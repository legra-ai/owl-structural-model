//! Deserialization enforces constructor invariants in text and binary formats.

use iri_rfc3987::Iri;
use owl_structural_model::{
    AnnotationValue, Axiom, AxiomAnnotations, ClassExpression, OntologyId, OntologyPatch,
    OntologyPatchOperation, OwlLiteral,
};
use serde::{Serialize, de::DeserializeOwned};

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}

fn assert_rejected<T: DeserializeOwned, S: Serialize>(invalid: &S) {
    let json = serde_json::to_string(invalid).unwrap();
    let bytes = postcard::to_allocvec(invalid).unwrap();
    assert!(serde_json::from_str::<T>(&json).is_err(), "accepted {json}");
    assert!(
        postcard::from_bytes::<T>(&bytes).is_err(),
        "accepted binary input"
    );
}

#[derive(Serialize)]
struct IdentityInput {
    ontology_iri: Option<Iri<String>>,
    version_iri: Option<Iri<String>>,
}

#[derive(Serialize)]
struct PatchInput {
    target: OntologyId,
    operations: Vec<OntologyPatchOperation>,
}

#[test]
fn version_only_identity_is_rejected_in_both_formats() {
    assert_rejected::<OntologyId, _>(&IdentityInput {
        ontology_iri: None,
        version_iri: Some(iri("urn:version")),
    });
}

#[test]
fn empty_patch_is_rejected_in_both_formats() {
    assert_rejected::<OntologyPatch, _>(&PatchInput {
        target: OntologyId::anonymous(),
        operations: vec![],
    });
}

#[test]
fn empty_language_tag_is_rejected_in_both_formats() {
    assert_rejected::<OwlLiteral, _>(&OwlLiteral::LanguageTagged {
        lexical: "hello".to_owned(),
        language: String::new(),
    });
}

#[test]
fn empty_anonymous_annotation_is_rejected_in_both_formats() {
    assert_rejected::<AnnotationValue, _>(&AnnotationValue::AnonymousIndividual(String::new()));
}

#[test]
fn nested_literal_validation_survives_patch_deserialization() {
    let invalid = OwlLiteral::LanguageTagged {
        lexical: "hello".to_owned(),
        language: String::new(),
    };
    let patch = OntologyPatch::try_new(
        OntologyId::anonymous(),
        vec![OntologyPatchOperation::AddAnnotation {
            annotation: owl_structural_model::Annotation::new(
                iri("urn:label"),
                AnnotationValue::Literal(invalid),
            ),
        }],
    )
    .unwrap();
    assert_rejected::<OntologyPatch, _>(&patch);
}

#[test]
fn empty_class_operands_are_rejected_in_both_formats() {
    assert_rejected::<ClassExpression, _>(&ClassExpression::ObjectUnionOf(vec![]));
    assert_rejected::<ClassExpression, _>(&ClassExpression::ObjectIntersectionOf(vec![]));
}

#[test]
fn short_class_axioms_are_rejected_in_both_formats() {
    // bounded: the two invalid arities of class equivalence/disjointness axioms.
    for classes in [vec![], vec![ClassExpression::Thing]] {
        assert_rejected::<Axiom, _>(&Axiom::EquivalentClasses {
            classes: classes.clone(),
            annotations: AxiomAnnotations::empty(),
        });
        assert_rejected::<Axiom, _>(&Axiom::DisjointClasses {
            classes,
            annotations: AxiomAnnotations::empty(),
        });
    }
}
