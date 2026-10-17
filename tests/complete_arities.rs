//! New expression and assertion arities are enforced in both Serde formats.

use iri_rfc3987::Iri;
use owl_structural_model::Individual;
use serde::{Serialize, Serializer, de::DeserializeOwned};

type Axiom = owl_structural_model::Axiom;
type AxiomAnnotations = owl_structural_model::AxiomAnnotations;
type ClassExpression = owl_structural_model::ClassExpression;
type DataRange = owl_structural_model::DataRange;

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}

struct VariantInput<T> {
    index: u32,
    name: &'static str,
    payload: T,
}
impl<T: Serialize> Serialize for VariantInput<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_variant(
            "MalformedStructuralInput",
            self.index,
            self.name,
            &self.payload,
        )
    }
}

fn assert_rejected<T: DeserializeOwned>(input: &impl Serialize) {
    let json = serde_json::to_string(input).unwrap();
    let bytes = postcard::to_allocvec(input).unwrap();
    assert!(serde_json::from_str::<T>(&json).is_err(), "accepted {json}");
    assert!(
        postcard::from_bytes::<T>(&bytes).is_err(),
        "accepted malformed binary operands"
    );
}

fn assert_accepted<T: DeserializeOwned + PartialEq + std::fmt::Debug>(
    input: &impl Serialize,
    expected: &T,
) {
    let json = serde_json::to_string(input).unwrap();
    let bytes = postcard::to_allocvec(input).unwrap();
    assert_eq!(&serde_json::from_str::<T>(&json).unwrap(), expected);
    assert_eq!(&postcard::from_bytes::<T>(&bytes).unwrap(), expected);
}

#[derive(Serialize)]
struct IndividualsInput {
    // bounded: at most one fixture individual, below the normative minimum.
    individuals: Vec<Individual>,
    annotations: AxiomAnnotations,
}

#[derive(Serialize)]
struct DisjointUnionInput {
    class: Iri<String>,
    // bounded: at most one fixture class, below the normative minimum.
    classes: Vec<ClassExpression>,
    annotations: AxiomAnnotations,
}

#[derive(Serialize)]
struct DataRestrictionInput {
    // bounded: empty fixture property list, below the normative minimum.
    properties: Vec<Iri<String>>,
    range: DataRange,
}

#[test]
fn individual_assertions_require_two_source_occurrences() {
    let valid = vec![
        Individual::named(iri("urn:first")),
        Individual::named(iri("urn:second")),
    ];
    for (index, name, expected) in [
        (
            26,
            "SameIndividual",
            Axiom::same_individual(valid.clone()).unwrap(),
        ),
        (
            27,
            "DifferentIndividuals",
            Axiom::different_individuals(valid.clone()).unwrap(),
        ),
    ] {
        assert_accepted(
            &VariantInput {
                index,
                name,
                payload: IndividualsInput {
                    individuals: valid.clone(),
                    annotations: AxiomAnnotations::empty(),
                },
            },
            &expected,
        );
    }
    // bounded: the two invalid occurrence counts for Same/DifferentIndividual.
    for individuals in [vec![], vec![Individual::named(iri("urn:individual"))]] {
        assert!(Axiom::same_individual(individuals.clone()).is_err());
        assert!(Axiom::different_individuals(individuals.clone()).is_err());
        for (index, name) in [(26, "SameIndividual"), (27, "DifferentIndividuals")] {
            assert_rejected::<Axiom>(&VariantInput {
                index,
                name,
                payload: IndividualsInput {
                    individuals: individuals.clone(),
                    annotations: AxiomAnnotations::empty(),
                },
            });
        }
    }
}

#[test]
fn disjoint_unions_require_two_source_occurrences() {
    let valid = vec![ClassExpression::thing(), ClassExpression::nothing()];
    assert_accepted(
        &VariantInput {
            index: 4,
            name: "DisjointUnion",
            payload: DisjointUnionInput {
                class: iri("urn:union"),
                classes: valid.clone(),
                annotations: AxiomAnnotations::empty(),
            },
        },
        &Axiom::disjoint_union(iri("urn:union"), valid).unwrap(),
    );
    // bounded: the two invalid occurrence counts for DisjointUnion.
    for classes in [vec![], vec![ClassExpression::thing()]] {
        assert!(Axiom::disjoint_union(iri("urn:union"), classes.clone()).is_err());
        assert_rejected::<Axiom>(&VariantInput {
            index: 4,
            name: "DisjointUnion",
            payload: DisjointUnionInput {
                class: iri("urn:union"),
                classes,
                annotations: AxiomAnnotations::empty(),
            },
        });
    }
}

#[test]
fn individual_enumerations_cannot_be_empty() {
    let valid = vec![Individual::named(iri("urn:individual"))];
    assert_accepted(
        &VariantInput {
            index: 4,
            name: "ObjectOneOf",
            payload: valid.clone(),
        },
        &ClassExpression::object_one_of(valid).unwrap(),
    );
    assert!(ClassExpression::object_one_of(vec![]).is_err());
    assert_rejected::<ClassExpression>(&VariantInput {
        index: 4,
        name: "ObjectOneOf",
        payload: Vec::<Individual>::new(),
    });
}

#[test]
fn data_restrictions_cannot_have_zero_property_positions() {
    for (index, name) in [(12, "DataSomeValuesFrom"), (13, "DataAllValuesFrom")] {
        let properties = vec![iri("urn:property")];
        let range = DataRange::datatype(iri("urn:datatype"));
        let expected = if index == 12 {
            ClassExpression::data_some_values_from(properties.clone(), range.clone()).unwrap()
        } else {
            ClassExpression::data_all_values_from(properties.clone(), range.clone()).unwrap()
        };
        assert_accepted(
            &VariantInput {
                index,
                name,
                payload: DataRestrictionInput { properties, range },
            },
            &expected,
        );
        assert_rejected::<ClassExpression>(&VariantInput {
            index,
            name,
            payload: DataRestrictionInput {
                properties: vec![],
                range: DataRange::datatype(iri("urn:datatype")),
            },
        });
    }
}
