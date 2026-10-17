//! Unknown meaning-bearing fields must not disappear during deserialization.

use owl_structural_model::{Entity, OntologyId};

type Annotation = owl_structural_model::Annotation;
type OntologyPatch = owl_structural_model::OntologyPatch;

#[test]
fn semantic_objects_reject_unknown_fields() {
    assert!(
        serde_json::from_str::<Entity>(r#"{"kind":"Class","iri":"urn:C","unknown":"lost"}"#)
            .is_err()
    );
    assert!(
        serde_json::from_str::<OntologyId>(
            r#"{"ontology_iri":null,"version_iri":null,"unknown":"lost"}"#
        )
        .is_err()
    );
    assert!(serde_json::from_str::<Annotation>(r#"{"annotations":{"annotations":[]},"property":"urn:p","value":{"Iri":"urn:v"},"unknown":"lost"}"#).is_err());
    assert!(serde_json::from_str::<OntologyPatch>(r#"{"target":{"ontology_iri":null,"version_iri":null},"operations":[{"AddImport":{"import_iri":"urn:import"}}],"unknown":"lost"}"#).is_err());
}

#[test]
fn unknown_nested_annotations_are_rejected() {
    let input = r#"{"annotations":{"annotations":[{"annotations":{"annotations":[]},"property":"urn:p","value":{"Iri":"urn:v"},"unknown":"lost"}]},"property":"urn:outer","value":{"Iri":"urn:outer-value"}}"#;
    assert!(serde_json::from_str::<Annotation>(input).is_err());
}
