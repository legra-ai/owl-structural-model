//! Every one of the 37 W3C OWL 2 axiom families round-trips in a nested patch.
mod inventory;
mod support;
use std::collections::HashSet;

use inventory::annotations;
use iri_rfc3987::Iri;
use owl_structural_model::OntologyId;
type OntologyPatch = owl_structural_model::OntologyPatch;
type OntologyPatchOperation = owl_structural_model::OntologyPatchOperation;

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}

#[test]
fn every_standard_axiom_family_round_trips_in_a_nested_patch() {
    // bounded: one distinct kind per standard fixture, at most 37 entries.
    let mut seen = HashSet::new();
    for (kind, construct) in inventory::all() {
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
