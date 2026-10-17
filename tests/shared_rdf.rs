//! Shared RDF identity and caller-selected values survive nested OWL models.

mod support;

use iri_rfc3987::Iri;
use owl_structural_model::DataRange;
use owl_structural_model::{
    Annotation, AnnotationSubject, AnnotationValue, Axiom, AxiomAnnotations, FacetRestriction,
    OntologyId, OntologyPatch, OntologyPatchOperation, OwlLiteral,
};
use owl_structural_model::{ClassExpression, Individual};
use rdf_structural_model::{BaseDirection, BlankNodeId, LanguageTag, Literal, LiteralMetadata};
use serde::{Deserialize, Serialize};

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}

fn blank(scope: &str) -> BlankNodeId {
    BlankNodeId::try_new(iri(scope), "same-local-label".to_owned()).unwrap()
}

#[test]
fn shared_literals_flow_directly_into_nested_axioms() {
    let literal = Literal::directional(
        "exact\nlexical text",
        LanguageTag::try_new("ar-EG".to_owned()).unwrap(),
        BaseDirection::Rtl,
    );
    let annotation: Annotation =
        Annotation::new(iri("urn:label"), AnnotationValue::Literal(literal.clone()));
    let range = DataRange::data_union_of(vec![
        DataRange::data_one_of(vec![literal.clone()]).unwrap(),
        DataRange::datatype_restriction(
            iri("urn:datatype"),
            vec![FacetRestriction::new(iri("urn:facet"), literal)],
        )
        .unwrap(),
    ])
    .unwrap();
    let axiom = Axiom::data_property_range(iri("urn:property"), range)
        .with_annotations(AxiomAnnotations::new(vec![annotation]));
    let patch = OntologyPatch::try_new(
        OntologyId::named(iri("urn:ontology")),
        vec![OntologyPatchOperation::AddAxiom { axiom }],
    )
    .unwrap();
    support::assert_round_trip(&patch);
}

#[test]
fn anonymous_individual_identity_includes_scope_in_subjects_and_values() {
    let first = blank("urn:first-source");
    let second = blank("urn:second-source");
    assert_ne!(first, second);
    let first_subject = AnnotationSubject::AnonymousIndividual(first.clone());
    let second_subject = AnnotationSubject::AnonymousIndividual(second.clone());
    assert_ne!(first_subject, second_subject);
    let first_value: AnnotationValue = AnnotationValue::anonymous_individual(first);
    let second_value: AnnotationValue = AnnotationValue::anonymous_individual(second);
    assert_ne!(first_value, second_value);
    let axiom: Axiom = Axiom::AnnotationAssertion {
        property: iri("urn:annotation"),
        subject: first_subject,
        value: second_value,
        annotations: AxiomAnnotations::empty(),
    };
    support::assert_round_trip(&axiom);
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum ExternalLiteral {
    Lexical(Literal),
    Binary {
        metadata: LiteralMetadata,
        digest: [u8; 32],
        byte_length: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct InternedAnonymous {
    scope: u64,
    local: u64,
}

#[test]
fn custom_representations_propagate_through_ranges_annotations_axioms_and_patches() {
    let literal = ExternalLiteral::Binary {
        metadata: Literal::directional(
            "metadata only",
            LanguageTag::try_new("en-US".to_owned()).unwrap(),
            BaseDirection::Ltr,
        )
        .metadata()
        .clone(),
        digest: [7; 32],
        byte_length: 1_000_000_000_000,
    };
    let annotations = AxiomAnnotations::new(vec![Annotation::new(
        iri("urn:origin"),
        AnnotationValue::AnonymousIndividual(InternedAnonymous {
            scope: 42,
            local: 7,
        }),
    )]);
    let data_axiom = Axiom::datatype_definition(
        iri("urn:datatype"),
        DataRange::data_intersection_of(vec![
            DataRange::data_one_of(vec![literal.clone()]).unwrap(),
            DataRange::datatype_restriction(
                iri("urn:datatype"),
                vec![FacetRestriction::new(iri("urn:facet"), literal.clone())],
            )
            .unwrap(),
        ])
        .unwrap(),
    )
    .with_annotations(annotations);
    let annotation_axiom = Axiom::AnnotationAssertion {
        property: iri("urn:annotation"),
        subject: AnnotationSubject::AnonymousIndividual(InternedAnonymous {
            scope: 43,
            local: 7,
        }),
        value: AnnotationValue::Literal(literal),
        annotations: AxiomAnnotations::empty(),
    };
    let patch = OntologyPatch::try_new(
        OntologyId::anonymous(),
        vec![
            OntologyPatchOperation::AddAxiom { axiom: data_axiom },
            OntologyPatchOperation::AddAxiom {
                axiom: annotation_axiom,
            },
        ],
    )
    .unwrap();
    support::assert_round_trip(&patch);
    // The default profile cannot decode a caller's selected payload type.
    let json = serde_json::to_string(&patch).unwrap();
    assert!(serde_json::from_str::<OntologyPatch>(&json).is_err());
    let bytes = postcard::to_allocvec(&patch).unwrap();
    assert!(postcard::from_bytes::<OntologyPatch>(&bytes).is_err());
}

#[test]
fn nested_literal_payloads_can_be_borrowed_from_binary_input() {
    let patch: OntologyPatch = OntologyPatch::try_new(
        OntologyId::anonymous(),
        vec![OntologyPatchOperation::AddAnnotation {
            annotation: Annotation::new(
                iri("urn:label"),
                AnnotationValue::Literal(Literal::directional(
                    "borrow this payload",
                    LanguageTag::try_new("en".to_owned()).unwrap(),
                    BaseDirection::Ltr,
                )),
            ),
        }],
    )
    .unwrap();
    let bytes = postcard::to_allocvec(&patch).unwrap();
    let borrowed: OntologyPatch<Literal<&str>, BlankNodeId<&str>> =
        postcard::from_bytes(&bytes).unwrap();
    let OntologyPatchOperation::AddAnnotation { annotation } = &borrowed.operations()[0] else {
        panic!("expected annotation");
    };
    let AnnotationValue::Literal(literal) = annotation.value() else {
        panic!("expected literal");
    };
    assert_eq!(literal.lexical(), "borrow this payload");
    let payload = literal.lexical().as_ptr() as usize;
    let start = bytes.as_ptr() as usize;
    assert!(payload >= start && payload + literal.lexical().len() <= start + bytes.len());
    assert_eq!(postcard::to_allocvec(&borrowed).unwrap(), bytes);
}

#[test]
fn constructors_do_not_require_payload_traits() {
    struct LiteralHandle(u64);
    struct AnonymousHandle(u64);
    let facet = FacetRestriction::new(iri("urn:facet"), LiteralHandle(8));
    assert_eq!(facet.value().0, 8);
    let annotation = Annotation::<LiteralHandle, AnonymousHandle>::new(
        iri("urn:annotation"),
        AnnotationValue::anonymous_individual(AnonymousHandle(9)),
    );
    let AnnotationValue::AnonymousIndividual(handle) = annotation.value() else {
        panic!("expected anonymous handle");
    };
    assert_eq!(handle.0, 9);
    let axiom = Axiom::data_property_range(
        iri("urn:property"),
        DataRange::datatype_restriction(iri("urn:datatype"), vec![facet]).unwrap(),
    )
    .with_annotations(AxiomAnnotations::new(vec![annotation]));
    let patch = OntologyPatch::try_new(
        OntologyId::anonymous(),
        vec![OntologyPatchOperation::AddAxiom { axiom }],
    )
    .unwrap();
    assert_eq!(patch.operations().len(), 1);
    assert!(AxiomAnnotations::<LiteralHandle, AnonymousHandle>::default().is_empty());
    // The public alias is the shared type itself, without a conversion layer.
    let _: OwlLiteral = Literal::string("shared value");
}

#[test]
fn custom_payloads_and_individuals_propagate_through_class_expressions() {
    let literal = ExternalLiteral::Lexical(Literal::string("custom literal"));
    let class = ClassExpression::object_intersection_of(vec![
        ClassExpression::data_has_value(iri("urn:p"), literal.clone()),
        ClassExpression::object_has_value(
            owl_structural_model::ObjectPropertyExpression::named(iri("urn:q")),
            Individual::anonymous(InternedAnonymous {
                scope: 11,
                local: 42,
            }),
        ),
        ClassExpression::object_one_of(vec![Individual::anonymous(InternedAnonymous {
            scope: 12,
            local: 42,
        })])
        .unwrap(),
        ClassExpression::data_some_values_from(
            vec![iri("urn:r")],
            DataRange::data_one_of(vec![literal]).unwrap(),
        )
        .unwrap(),
    ])
    .unwrap();
    let patch = OntologyPatch::try_new(
        OntologyId::anonymous(),
        vec![OntologyPatchOperation::AddAxiom {
            axiom: Axiom::class_assertion(
                class,
                Individual::anonymous(InternedAnonymous {
                    scope: 13,
                    local: 42,
                }),
            ),
        }],
    )
    .unwrap();
    support::assert_round_trip(&patch);
}

#[test]
fn class_assertions_borrow_nested_literal_and_anonymous_payloads() {
    let patch: OntologyPatch = OntologyPatch::try_new(
        OntologyId::anonymous(),
        vec![OntologyPatchOperation::AddAxiom {
            axiom: Axiom::class_assertion(
                ClassExpression::data_has_value(
                    iri("urn:p"),
                    Literal::string("borrow class payload"),
                ),
                Individual::anonymous(blank("urn:scope")),
            ),
        }],
    )
    .unwrap();
    let bytes = postcard::to_allocvec(&patch).unwrap();
    let borrowed: OntologyPatch<Literal<&str>, BlankNodeId<&str>> =
        postcard::from_bytes(&bytes).unwrap();
    let OntologyPatchOperation::AddAxiom {
        axiom: Axiom::ClassAssertion {
            class, individual, ..
        },
    } = &borrowed.operations()[0]
    else {
        panic!("expected a class assertion");
    };
    let ClassExpression::DataHasValue { value, .. } = class else {
        panic!("expected a data value");
    };
    let Individual::Anonymous(id) = individual else {
        panic!("expected an anonymous individual");
    };
    assert_eq!(id.scope().as_str(), "urn:scope");
    assert_eq!(id.label(), "same-local-label");
    // Both identity and literal text are lent from the serialized input buffer.
    for payload in [value.lexical(), id.label()] {
        let pointer = payload.as_ptr() as usize;
        let start = bytes.as_ptr() as usize;
        assert!(pointer >= start && pointer + payload.len() <= start + bytes.len());
    }
    assert_eq!(postcard::to_allocvec(&borrowed).unwrap(), bytes);
}
