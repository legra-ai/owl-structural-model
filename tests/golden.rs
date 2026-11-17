//! Exact JSON and Postcard encodings of one annotated value per axiom family.
//!
//! Regenerate the files only with `UPDATE_GOLDEN=1 cargo test --test golden`.

mod inventory;

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use owl_structural_model::AxiomKind;

type Axiom = owl_structural_model::Axiom;

/// The pair of golden files recording one axiom family's encodings.
struct GoldenFiles {
    json: PathBuf,
    postcard: PathBuf,
}

impl GoldenFiles {
    fn for_kind(kind: AxiomKind) -> Self {
        let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/golden");
        let stem = kind.functional_syntax_production();
        Self {
            json: directory.join(format!("{stem}.json")),
            postcard: directory.join(format!("{stem}.postcard")),
        }
    }

    fn update_requested() -> bool {
        std::env::var_os("UPDATE_GOLDEN").is_some_and(|value| value == "1")
    }

    fn write(&self, json: &str, bytes: &[u8]) {
        fs::create_dir_all(self.json.parent().unwrap()).unwrap();
        fs::write(&self.json, format!("{json}\n")).unwrap();
        fs::write(&self.postcard, bytes).unwrap();
    }

    fn assert_matches(&self, axiom: &Axiom, json: &str, bytes: &[u8]) {
        let golden_json = fs::read_to_string(&self.json)
            .unwrap_or_else(|error| panic!("{}: {error}", self.json.display()));
        let golden_bytes = fs::read(&self.postcard)
            .unwrap_or_else(|error| panic!("{}: {error}", self.postcard.display()));
        assert_eq!(golden_json, format!("{json}\n"), "{}", self.json.display());
        assert_eq!(golden_bytes, bytes, "{}", self.postcard.display());
        assert_eq!(&serde_json::from_str::<Axiom>(&golden_json).unwrap(), axiom);
        assert_eq!(
            &postcard::from_bytes::<Axiom>(&golden_bytes).unwrap(),
            axiom
        );
    }
}

#[test]
fn every_axiom_family_matches_its_golden_encodings() {
    // bounded: one distinct kind per standard fixture, at most 37 entries.
    let mut seen = HashSet::new();
    for (kind, construct) in inventory::all() {
        assert!(seen.insert(kind), "duplicate fixture: {kind:?}");
        let axiom = construct().with_annotations(inventory::annotations());
        let json = serde_json::to_string(&axiom).unwrap();
        let bytes = postcard::to_allocvec(&axiom).unwrap();
        let files = GoldenFiles::for_kind(kind);
        if GoldenFiles::update_requested() {
            files.write(&json, &bytes);
        }
        files.assert_matches(&axiom, &json, &bytes);
    }
    assert_eq!(seen.len(), AxiomKind::ALL.len());
}
