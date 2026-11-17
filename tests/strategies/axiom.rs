//! Generators for data, assertion and annotation axioms, and for every axiom family.

use owl_structural_model::{AnnotationSubject, AxiomKind, KeyProperties};
use proptest::prelude::*;
use proptest::strategy::Union;

use super::axiom_schema::{Axiom, schema_axioms};
use super::expression::{class_expression, data_range};
use super::leaf::{
    MAX_OPERANDS, annotation_property, annotation_value, annotations, blank, individual, iri,
    literal, object_property,
};

fn data_properties() -> impl Strategy<Value = Vec<iri_rfc3987::Iri<String>>> {
    prop::collection::vec(iri(), 2..=MAX_OPERANDS)
}

fn individuals() -> impl Strategy<Value = Vec<owl_structural_model::Individual>> {
    prop::collection::vec(individual(), 2..=MAX_OPERANDS)
}

fn key() -> impl Strategy<Value = KeyProperties> {
    (
        prop::collection::vec(object_property(), 0..MAX_OPERANDS),
        prop::collection::vec(iri(), 0..MAX_OPERANDS),
    )
        .prop_filter_map("a key needs a property", |(objects, data)| {
            KeyProperties::try_new(objects, data).ok()
        })
}

fn subject() -> impl Strategy<Value = AnnotationSubject> {
    prop_oneof![
        iri().prop_map(AnnotationSubject::Iri),
        blank().prop_map(AnnotationSubject::AnonymousIndividual),
    ]
}

fn data_axioms() -> Vec<BoxedStrategy<Axiom>> {
    vec![
        (iri(), iri())
            .prop_map(|(sub, sup)| Axiom::sub_data_property_of(sub, sup))
            .boxed(),
        data_properties()
            .prop_map(|p| Axiom::equivalent_data_properties(p).unwrap())
            .boxed(),
        data_properties()
            .prop_map(|p| Axiom::disjoint_data_properties(p).unwrap())
            .boxed(),
        (iri(), class_expression())
            .prop_map(|(property, class)| Axiom::data_property_domain(property, class))
            .boxed(),
        (iri(), data_range())
            .prop_map(|(property, range)| Axiom::data_property_range(property, range))
            .boxed(),
        iri().prop_map(Axiom::functional_data_property).boxed(),
        (iri(), data_range())
            .prop_map(|(datatype, range)| Axiom::datatype_definition(datatype, range))
            .boxed(),
        (class_expression(), key())
            .prop_map(|(class, key)| Axiom::has_key(class, key))
            .boxed(),
    ]
}

fn assertion_axioms() -> Vec<BoxedStrategy<Axiom>> {
    vec![
        individuals()
            .prop_map(|i| Axiom::same_individual(i).unwrap())
            .boxed(),
        individuals()
            .prop_map(|i| Axiom::different_individuals(i).unwrap())
            .boxed(),
        (class_expression(), individual())
            .prop_map(|(class, member)| Axiom::class_assertion(class, member))
            .boxed(),
        (object_property(), individual(), individual())
            .prop_map(|(p, s, t)| Axiom::object_property_assertion(p, s, t))
            .boxed(),
        (object_property(), individual(), individual())
            .prop_map(|(p, s, t)| Axiom::negative_object_property_assertion(p, s, t))
            .boxed(),
        (iri(), individual(), literal())
            .prop_map(|(p, s, v)| Axiom::data_property_assertion(p, s, v))
            .boxed(),
        (iri(), individual(), literal())
            .prop_map(|(p, s, v)| Axiom::negative_data_property_assertion(p, s, v))
            .boxed(),
    ]
}

fn annotation_axioms() -> Vec<BoxedStrategy<Axiom>> {
    vec![
        (annotation_property(), subject(), annotation_value())
            .prop_map(|(p, s, v)| Axiom::annotation_assertion(p, s, v))
            .boxed(),
        (annotation_property(), annotation_property())
            .prop_map(|(sub, sup)| Axiom::sub_annotation_property_of(sub, sup))
            .boxed(),
        (annotation_property(), iri())
            .prop_map(|(p, domain)| Axiom::annotation_property_domain(p, domain))
            .boxed(),
        (annotation_property(), iri())
            .prop_map(|(p, range)| Axiom::annotation_property_range(p, range))
            .boxed(),
    ]
}

/// Annotated axioms of every one of the 37 standard families.
pub fn axiom() -> impl Strategy<Value = Axiom> {
    // bounded: one strategy per standard axiom family.
    let families: Vec<_> = schema_axioms()
        .into_iter()
        .chain(data_axioms())
        .chain(assertion_axioms())
        .chain(annotation_axioms())
        .collect();
    assert_eq!(families.len(), AxiomKind::ALL.len());
    (Union::new(families), annotations())
        .prop_map(|(axiom, annotations)| axiom.with_annotations(annotations))
}
