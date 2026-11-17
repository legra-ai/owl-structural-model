//! Property-based serialization checks over bounded-depth generated values.

mod strategies;

use std::fmt::Debug;

use owl_structural_model::{AxiomKind, OwlLiteral};
use proptest::prelude::*;
use serde::ser::SerializeStructVariant;
use serde::{Serialize, Serializer, de::DeserializeOwned};

type Axiom = owl_structural_model::Axiom;
type AxiomAnnotations = owl_structural_model::AxiomAnnotations;
type ClassExpression = owl_structural_model::ClassExpression;
type DataRange = owl_structural_model::DataRange;

fn round_trips<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: &T) {
    let json = serde_json::to_string(value).unwrap();
    assert_eq!(&serde_json::from_str::<T>(&json).unwrap(), value);
    let bytes = postcard::to_allocvec(value).unwrap();
    assert_eq!(&postcard::from_bytes::<T>(&bytes).unwrap(), value);
}

/// Insert an unknown member into the object of an externally tagged struct variant.
fn with_unknown_field(json: &str) -> Option<String> {
    let (tag, body) = json.strip_prefix('{')?.split_once(':')?;
    let members = body.strip_prefix('{')?;
    Some(format!("{{{tag}:{{\"unknown\":0,{members}"))
}

fn rejects_unknown_fields<T: Serialize + DeserializeOwned + Debug>(value: &T) {
    let json = serde_json::to_string(value).unwrap();
    if let Some(tampered) = with_unknown_field(&json) {
        let decoded = serde_json::from_str::<T>(&tampered);
        assert!(decoded.is_err(), "accepted {tampered}: {decoded:?}");
    }
}

/// A too-short operand list in a newtype or struct variant of a structural enum.
struct ShortOperands<T> {
    enumeration: &'static str,
    index: u32,
    variant: &'static str,
    field: Option<&'static str>,
    operands: Vec<T>,
}

impl<T: Serialize> Serialize for ShortOperands<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Some(field) = self.field else {
            return serializer.serialize_newtype_variant(
                self.enumeration,
                self.index,
                self.variant,
                &self.operands,
            );
        };
        let mut value =
            serializer.serialize_struct_variant(self.enumeration, self.index, self.variant, 2)?;
        value.serialize_field(field, &self.operands)?;
        value.serialize_field("annotations", &AxiomAnnotations::empty())?;
        value.end()
    }
}

impl<T: Serialize> ShortOperands<T> {
    fn axiom(kind: AxiomKind, field: &'static str, operands: Vec<T>) -> Self {
        let index = AxiomKind::ALL
            .iter()
            .position(|candidate| *candidate == kind)
            .unwrap();
        Self {
            enumeration: "Axiom",
            index: u32::try_from(index).unwrap(),
            variant: kind.functional_syntax_production(),
            field: Some(field),
            operands,
        }
    }

    fn assert_rejected<D: DeserializeOwned + Debug>(&self) {
        let json = serde_json::to_string(self).unwrap();
        assert!(serde_json::from_str::<D>(&json).is_err(), "accepted {json}");
        let bytes = postcard::to_allocvec(self).unwrap();
        assert!(
            postcard::from_bytes::<D>(&bytes).is_err(),
            "accepted binary {json}"
        );
    }
}

fn newtype<T>(
    enumeration: &'static str,
    index: u32,
    variant: &'static str,
    operands: Vec<T>,
) -> ShortOperands<T> {
    ShortOperands {
        enumeration,
        index,
        variant,
        field: None,
        operands,
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    #[test]
    fn class_expressions_round_trip(value in strategies::class_expression()) {
        round_trips(&value);
        rejects_unknown_fields(&value);
    }

    #[test]
    fn data_ranges_round_trip(value in strategies::data_range()) {
        round_trips(&value);
        rejects_unknown_fields(&value);
    }

    #[test]
    fn axioms_round_trip(value in strategies::axiom()) {
        round_trips(&value);
        let json = serde_json::to_string(&value).unwrap();
        prop_assert!(with_unknown_field(&json).is_some(), "axioms are struct variants: {json}");
        rejects_unknown_fields(&value);
    }

    #[test]
    fn short_boolean_class_expressions_are_rejected(
        classes in prop::collection::vec(strategies::class_expression(), 0..2),
    ) {
        newtype("ClassExpression", 1, "ObjectIntersectionOf", classes.clone()).assert_rejected::<ClassExpression>();
        newtype("ClassExpression", 2, "ObjectUnionOf", classes).assert_rejected::<ClassExpression>();
        newtype("ClassExpression", 4, "ObjectOneOf", Vec::<owl_structural_model::Individual>::new()).assert_rejected::<ClassExpression>();
    }

    #[test]
    fn short_data_ranges_are_rejected(
        ranges in prop::collection::vec(strategies::data_range(), 0..2),
    ) {
        newtype("DataRange", 1, "DataIntersectionOf", ranges.clone()).assert_rejected::<DataRange>();
        newtype("DataRange", 2, "DataUnionOf", ranges).assert_rejected::<DataRange>();
        newtype("DataRange", 4, "DataOneOf", Vec::<OwlLiteral>::new()).assert_rejected::<DataRange>();
    }

    #[test]
    fn short_n_ary_axioms_are_rejected(
        classes in prop::collection::vec(strategies::class_expression(), 0..2),
        properties in prop::collection::vec(strategies::object_property(), 0..2),
        individuals in prop::collection::vec(strategies::individual(), 0..2),
    ) {
        for kind in [AxiomKind::EquivalentClasses, AxiomKind::DisjointClasses] {
            ShortOperands::axiom(kind, "classes", classes.clone()).assert_rejected::<Axiom>();
        }
        for kind in [AxiomKind::EquivalentObjectProperties, AxiomKind::DisjointObjectProperties] {
            ShortOperands::axiom(kind, "properties", properties.clone()).assert_rejected::<Axiom>();
        }
        for kind in [AxiomKind::SameIndividual, AxiomKind::DifferentIndividuals] {
            ShortOperands::axiom(kind, "individuals", individuals.clone()).assert_rejected::<Axiom>();
        }
    }
}
