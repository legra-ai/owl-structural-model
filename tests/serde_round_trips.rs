//! Nested semantic values preserve their structure in JSON and Postcard.

mod support;

use iri_rfc3987::Iri;
use owl_structural_model::{
    ClassExpression, Entity, EntityKind, ObjectPropertyExpression, OntologyId, OwlLiteral,
    SubObjectPropertyExpression,
};
type Annotation = owl_structural_model::Annotation;
type AnnotationSubject = owl_structural_model::AnnotationSubject;
type AnnotationValue = owl_structural_model::AnnotationValue;
type Axiom = owl_structural_model::Axiom;
type AxiomAnnotations = owl_structural_model::AxiomAnnotations;
type OntologyPatch = owl_structural_model::OntologyPatch;
type OntologyPatchOperation = owl_structural_model::OntologyPatchOperation;

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}

#[test]
fn every_entity_kind_round_trips() {
    // bounded: all six supported OWL entity categories.
    for kind in [
        EntityKind::Class,
        EntityKind::ObjectProperty,
        EntityKind::DataProperty,
        EntityKind::AnnotationProperty,
        EntityKind::NamedIndividual,
        EntityKind::Datatype,
    ] {
        support::assert_round_trip(&Entity::new(kind, iri("urn:entity")));
    }
}

#[test]
fn every_class_expression_form_round_trips() {
    let named = ClassExpression::named(iri("urn:class"));
    let property =
        ObjectPropertyExpression::inverse_of(ObjectPropertyExpression::named(iri("urn:p")));
    // bounded: one example of each supported class-expression form.
    for expression in [
        named.clone(),
        ClassExpression::Thing,
        ClassExpression::Nothing,
        ClassExpression::object_intersection_of(vec![named.clone(), ClassExpression::Thing])
            .unwrap(),
        ClassExpression::object_union_of(vec![named.clone(), ClassExpression::Nothing]).unwrap(),
        ClassExpression::object_complement_of(named.clone()),
        ClassExpression::object_some_values_from(property.clone(), named.clone()),
        ClassExpression::object_all_values_from(property, named),
    ] {
        support::assert_round_trip(&expression);
    }
}

#[test]
fn every_ontology_identity_form_round_trips() {
    // bounded: anonymous, named and versioned identity forms.
    for identity in [
        OntologyId::anonymous(),
        OntologyId::named(iri("urn:ontology")),
        OntologyId::versioned(iri("urn:ontology"), iri("urn:version")),
    ] {
        support::assert_round_trip(&identity);
    }
}

#[test]
fn all_patch_operations_round_trip_with_nested_annotations() {
    let annotation = Annotation::new(
        iri("urn:label"),
        AnnotationValue::Literal(OwlLiteral::language_tagged(
            "café",
            rdf_structural_model::LanguageTag::try_new("fr".to_owned()).unwrap(),
        )),
    );
    let axiom = Axiom::declaration(Entity::class(iri("urn:class")))
        .with_annotations(AxiomAnnotations::new(vec![annotation.clone()]));
    let patch = OntologyPatch::try_new(
        OntologyId::versioned(iri("urn:ontology"), iri("urn:version")),
        vec![
            OntologyPatchOperation::AddImport {
                import_iri: iri("urn:import"),
            },
            OntologyPatchOperation::RemoveImport {
                import_iri: iri("urn:import"),
            },
            OntologyPatchOperation::AddAnnotation {
                annotation: annotation.clone(),
            },
            OntologyPatchOperation::RemoveAnnotation { annotation },
            OntologyPatchOperation::AddAxiom {
                axiom: axiom.clone(),
            },
            OntologyPatchOperation::RemoveAxiom { axiom },
        ],
    )
    .unwrap();
    support::assert_round_trip(&patch);
}

#[test]
fn class_axioms_and_annotation_forms_round_trip() {
    // bounded: the class axiom forms not exercised by the property-axiom suites.
    for axiom in [
        Axiom::sub_class_of(ClassExpression::Thing, ClassExpression::Nothing),
        Axiom::equivalent_classes(vec![ClassExpression::Thing, ClassExpression::Nothing]).unwrap(),
        Axiom::disjoint_classes(vec![ClassExpression::Thing, ClassExpression::Nothing]).unwrap(),
        Axiom::sub_object_property_of(
            SubObjectPropertyExpression::Property(ObjectPropertyExpression::named(iri("urn:p"))),
            ObjectPropertyExpression::named(iri("urn:q")),
        ),
    ] {
        support::assert_round_trip(&axiom);
    }
    let literal =
        OwlLiteral::typed("005", iri("http://www.w3.org/2001/XMLSchema#integer")).unwrap();
    // bounded: both annotation-subject forms.
    for subject in [
        AnnotationSubject::Iri(iri("urn:subject")),
        AnnotationSubject::AnonymousIndividual(
            rdf_structural_model::BlankNodeId::try_new(iri("urn:document"), "b0".to_owned())
                .unwrap(),
        ),
    ] {
        // bounded: all three annotation-value categories and both literal variants.
        for value in [
            AnnotationValue::Iri(iri("urn:value")),
            AnnotationValue::anonymous_individual(
                rdf_structural_model::BlankNodeId::try_new(iri("urn:document"), "b1".to_owned())
                    .unwrap(),
            ),
            AnnotationValue::Literal(literal.clone()),
            AnnotationValue::Literal(OwlLiteral::language_tagged(
                "bonjour",
                rdf_structural_model::LanguageTag::try_new("fr".to_owned()).unwrap(),
            )),
        ] {
            support::assert_round_trip(&Axiom::AnnotationAssertion {
                property: iri("urn:annotation"),
                subject: subject.clone(),
                value,
                annotations: AxiomAnnotations::empty(),
            });
        }
    }
}
