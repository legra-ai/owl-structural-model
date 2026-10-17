//! Independent fixture inventory of the 37 W3C OWL 2 axiom families.
mod support;
use iri_rfc3987::Iri;
use owl_structural_model::{
    AxiomKind, Entity, Individual, KeyProperties, ObjectPropertyExpression, OntologyId,
    SubObjectPropertyExpression,
};
use rdf_structural_model::{BlankNodeId, Literal};
use std::collections::HashSet;
type Axiom = owl_structural_model::Axiom;
type Annotation = owl_structural_model::Annotation;
type AnnotationSubject = owl_structural_model::AnnotationSubject;
type AnnotationValue = owl_structural_model::AnnotationValue;
type AxiomAnnotations = owl_structural_model::AxiomAnnotations;
type ClassExpression = owl_structural_model::ClassExpression;
type DataRange = owl_structural_model::DataRange;
type OntologyPatch = owl_structural_model::OntologyPatch;
type OntologyPatchOperation = owl_structural_model::OntologyPatchOperation;
fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}
fn class() -> ClassExpression {
    ClassExpression::named(iri("urn:C"))
}
fn property() -> ObjectPropertyExpression {
    ObjectPropertyExpression::named(iri("urn:p"))
}
fn range() -> DataRange {
    DataRange::datatype(iri("http://www.w3.org/2001/XMLSchema#string"))
}
fn literal() -> Literal {
    Literal::string("exact value")
}
fn blank() -> BlankNodeId {
    BlankNodeId::try_new(iri("urn:scope"), "b0".to_owned()).unwrap()
}
fn individual() -> Individual {
    Individual::named(iri("urn:individual"))
}
fn anonymous() -> Individual {
    Individual::anonymous(blank())
}
fn annotations() -> AxiomAnnotations {
    AxiomAnnotations::new(vec![
        Annotation::new(iri("urn:outer"), AnnotationValue::Literal(literal())).with_annotations(
            AxiomAnnotations::new(vec![Annotation::new(
                iri("urn:inner"),
                AnnotationValue::AnonymousIndividual(blank()),
            )]),
        ),
    ])
}
type Fixture = (AxiomKind, fn() -> Axiom);
fn class_axiom_fixtures() -> [Fixture; 5] {
    [
        (AxiomKind::Declaration, || {
            Axiom::declaration(Entity::class(iri("urn:C")))
        }),
        (AxiomKind::SubClassOf, || {
            Axiom::sub_class_of(class(), class())
        }),
        (AxiomKind::EquivalentClasses, || {
            Axiom::equivalent_classes(vec![class(), class()]).unwrap()
        }),
        (AxiomKind::DisjointClasses, || {
            Axiom::disjoint_classes(vec![class(), class()]).unwrap()
        }),
        (AxiomKind::DisjointUnion, || {
            Axiom::disjoint_union(iri("urn:Union"), vec![class(), class()]).unwrap()
        }),
    ]
}
fn property_axiom_fixtures() -> [Fixture; 8] {
    [
        (AxiomKind::SubObjectPropertyOf, || {
            Axiom::sub_object_property_of(
                SubObjectPropertyExpression::Property(property()),
                property(),
            )
        }),
        (AxiomKind::FunctionalObjectProperty, || {
            Axiom::functional_object_property(property())
        }),
        (AxiomKind::InverseFunctionalObjectProperty, || {
            Axiom::inverse_functional_object_property(property())
        }),
        (AxiomKind::ReflexiveObjectProperty, || {
            Axiom::reflexive_object_property(property())
        }),
        (AxiomKind::IrreflexiveObjectProperty, || {
            Axiom::irreflexive_object_property(property())
        }),
        (AxiomKind::SymmetricObjectProperty, || {
            Axiom::symmetric_object_property(property())
        }),
        (AxiomKind::AsymmetricObjectProperty, || {
            Axiom::asymmetric_object_property(property())
        }),
        (AxiomKind::TransitiveObjectProperty, || {
            Axiom::transitive_object_property(property())
        }),
    ]
}
fn property_schema_fixtures() -> [Fixture; 5] {
    [
        (AxiomKind::EquivalentObjectProperties, || {
            Axiom::equivalent_object_properties(vec![property(), property()]).unwrap()
        }),
        (AxiomKind::DisjointObjectProperties, || {
            Axiom::disjoint_object_properties(vec![property(), property()]).unwrap()
        }),
        (AxiomKind::InverseObjectProperties, || {
            Axiom::inverse_object_properties(property(), property())
        }),
        (AxiomKind::ObjectPropertyDomain, || {
            Axiom::object_property_domain(property(), class())
        }),
        (AxiomKind::ObjectPropertyRange, || {
            Axiom::object_property_range(property(), class())
        }),
    ]
}
fn data_axiom_fixtures() -> [Fixture; 8] {
    [
        (AxiomKind::SubDataPropertyOf, || {
            Axiom::sub_data_property_of(iri("urn:p"), iri("urn:q"))
        }),
        (AxiomKind::EquivalentDataProperties, || {
            Axiom::equivalent_data_properties(vec![iri("urn:p"), iri("urn:q")]).unwrap()
        }),
        (AxiomKind::DisjointDataProperties, || {
            Axiom::disjoint_data_properties(vec![iri("urn:p"), iri("urn:q")]).unwrap()
        }),
        (AxiomKind::DataPropertyDomain, || {
            Axiom::data_property_domain(iri("urn:p"), class())
        }),
        (AxiomKind::DataPropertyRange, || {
            Axiom::data_property_range(iri("urn:p"), range())
        }),
        (AxiomKind::FunctionalDataProperty, || {
            Axiom::functional_data_property(iri("urn:p"))
        }),
        (AxiomKind::DatatypeDefinition, || {
            Axiom::datatype_definition(iri("urn:datatype"), range())
        }),
        (AxiomKind::HasKey, || {
            Axiom::has_key(
                class(),
                KeyProperties::try_new(vec![property()], vec![]).unwrap(),
            )
        }),
    ]
}
fn assertion_fixtures() -> [Fixture; 7] {
    [
        (AxiomKind::SameIndividual, || {
            Axiom::same_individual(vec![individual(), anonymous()]).unwrap()
        }),
        (AxiomKind::DifferentIndividuals, || {
            Axiom::different_individuals(vec![individual(), anonymous()]).unwrap()
        }),
        (AxiomKind::ClassAssertion, || {
            Axiom::class_assertion(class(), anonymous())
        }),
        (AxiomKind::ObjectPropertyAssertion, || {
            Axiom::object_property_assertion(property(), individual(), anonymous())
        }),
        (AxiomKind::NegativeObjectPropertyAssertion, || {
            Axiom::negative_object_property_assertion(property(), individual(), anonymous())
        }),
        (AxiomKind::DataPropertyAssertion, || {
            Axiom::data_property_assertion(iri("urn:p"), anonymous(), literal())
        }),
        (AxiomKind::NegativeDataPropertyAssertion, || {
            Axiom::negative_data_property_assertion(iri("urn:p"), anonymous(), literal())
        }),
    ]
}
fn annotation_axiom_fixtures() -> [Fixture; 4] {
    [
        (AxiomKind::AnnotationAssertion, || {
            Axiom::annotation_assertion(
                iri("urn:label"),
                AnnotationSubject::AnonymousIndividual(blank()),
                AnnotationValue::Literal(literal()),
            )
        }),
        (AxiomKind::SubAnnotationPropertyOf, || {
            Axiom::sub_annotation_property_of(iri("urn:p"), iri("urn:q"))
        }),
        (AxiomKind::AnnotationPropertyDomain, || {
            Axiom::annotation_property_domain(iri("urn:p"), iri("urn:domain"))
        }),
        (AxiomKind::AnnotationPropertyRange, || {
            Axiom::annotation_property_range(iri("urn:p"), iri("urn:range"))
        }),
    ]
}
#[test]
fn every_standard_axiom_family_round_trips_in_a_nested_patch() {
    // bounded: 37 static fixture function pointers; construct only one axiom at a time.
    let fixtures = class_axiom_fixtures()
        .into_iter()
        .chain(property_axiom_fixtures())
        .chain(property_schema_fixtures())
        .chain(data_axiom_fixtures())
        .chain(assertion_fixtures())
        .chain(annotation_axiom_fixtures());
    // bounded: one distinct kind per standard fixture, at most 37 entries.
    let mut seen = HashSet::new();
    for (kind, construct) in fixtures {
        let axiom = construct();
        assert_eq!(axiom.kind(), kind);
        assert!(seen.insert(kind), "duplicate fixture: {kind:?}");
        let annotated = axiom.with_annotations(annotations());
        assert_eq!(annotated.annotations(), &annotations());
        let patch = OntologyPatch::try_new(
            OntologyId::named(iri("urn:ontology")),
            vec![
                OntologyPatchOperation::AddAxiom {
                    axiom: annotated.clone(),
                },
                OntologyPatchOperation::RemoveAxiom { axiom: annotated },
            ],
        )
        .unwrap();
        support::assert_round_trip(&patch);
    }
    assert_eq!(seen.len(), 37);
}
