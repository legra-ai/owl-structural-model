//! W3C class-expression coverage, cardinalities and qualified defaults.

mod support;

use std::collections::HashSet;

use iri_rfc3987::Iri;
use owl_structural_model::{
    Cardinality, ClassExpressionKind, Individual, ObjectPropertyExpression,
};
use rdf_structural_model::{BlankNodeId, Literal};
use serde::ser::SerializeStructVariant;
use serde::{Serialize, Serializer};

type ClassExpression = owl_structural_model::ClassExpression;
type DataRange = owl_structural_model::DataRange;

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}
fn class() -> ClassExpression {
    ClassExpression::named(iri("urn:class"))
}
fn property() -> ObjectPropertyExpression {
    ObjectPropertyExpression::named(iri("urn:property"))
}
fn individual() -> Individual {
    Individual::anonymous(BlankNodeId::try_new(iri("urn:scope"), "individual".to_owned()).unwrap())
}
fn range() -> DataRange {
    DataRange::datatype(iri("urn:datatype"))
}
fn cardinality() -> Cardinality {
    Cardinality::try_new("18446744073709551616000000000000000000000000000").unwrap()
}

type Fixture = (ClassExpressionKind, fn() -> ClassExpression);
fn basic_expressions() -> [Fixture; 9] {
    [
        (ClassExpressionKind::Named, class),
        (ClassExpressionKind::ObjectIntersectionOf, || {
            ClassExpression::object_intersection_of(vec![class(), ClassExpression::thing()])
                .unwrap()
        }),
        (ClassExpressionKind::ObjectUnionOf, || {
            ClassExpression::object_union_of(vec![class(), ClassExpression::nothing()]).unwrap()
        }),
        (ClassExpressionKind::ObjectComplementOf, || {
            ClassExpression::object_complement_of(class())
        }),
        (ClassExpressionKind::ObjectOneOf, || {
            ClassExpression::object_one_of(vec![individual()]).unwrap()
        }),
        (ClassExpressionKind::ObjectSomeValuesFrom, || {
            ClassExpression::object_some_values_from(property(), class())
        }),
        (ClassExpressionKind::ObjectAllValuesFrom, || {
            ClassExpression::object_all_values_from(property(), class())
        }),
        (ClassExpressionKind::ObjectHasValue, || {
            ClassExpression::object_has_value(property(), individual())
        }),
        (ClassExpressionKind::ObjectHasSelf, || {
            ClassExpression::object_has_self(property())
        }),
    ]
}
fn other_expressions() -> [Fixture; 9] {
    [
        (ClassExpressionKind::ObjectMinCardinality, || {
            ClassExpression::object_min_cardinality(cardinality(), property(), Some(class()))
        }),
        (ClassExpressionKind::ObjectMaxCardinality, || {
            ClassExpression::object_max_cardinality(cardinality(), property(), Some(class()))
        }),
        (ClassExpressionKind::ObjectExactCardinality, || {
            ClassExpression::object_exact_cardinality(cardinality(), property(), Some(class()))
        }),
        (ClassExpressionKind::DataSomeValuesFrom, || {
            ClassExpression::data_some_values_from(vec![iri("urn:property")], range()).unwrap()
        }),
        (ClassExpressionKind::DataAllValuesFrom, || {
            ClassExpression::data_all_values_from(vec![iri("urn:property")], range()).unwrap()
        }),
        (ClassExpressionKind::DataHasValue, || {
            ClassExpression::data_has_value(iri("urn:property"), Literal::string("exact lexical"))
        }),
        (ClassExpressionKind::DataMinCardinality, || {
            ClassExpression::data_min_cardinality(cardinality(), iri("urn:property"), Some(range()))
        }),
        (ClassExpressionKind::DataMaxCardinality, || {
            ClassExpression::data_max_cardinality(cardinality(), iri("urn:property"), Some(range()))
        }),
        (ClassExpressionKind::DataExactCardinality, || {
            ClassExpression::data_exact_cardinality(
                cardinality(),
                iri("urn:property"),
                Some(range()),
            )
        }),
    ]
}

#[test]
fn every_standard_class_expression_round_trips_recursively() {
    // bounded: the 18 standard OWL 2 class-expression forms.
    let mut seen = HashSet::new();
    for (kind, construct) in basic_expressions().into_iter().chain(other_expressions()) {
        let expression = construct();
        assert_eq!(expression.kind(), kind);
        assert!(seen.insert(kind));
        let nested = ClassExpression::object_union_of(vec![
            ClassExpression::object_complement_of(expression),
            ClassExpression::thing(),
        ])
        .unwrap();
        support::assert_round_trip(&nested);
    }
    assert_eq!(seen.len(), 18);
}

#[test]
fn unqualified_cardinalities_have_explicit_standard_fillers() {
    assert_eq!(
        ClassExpression::object_min_cardinality(0.into(), property(), None),
        ClassExpression::object_min_cardinality(
            0.into(),
            property(),
            Some(ClassExpression::thing())
        )
    );
    assert_eq!(
        ClassExpression::object_max_cardinality(1.into(), property(), None),
        ClassExpression::object_max_cardinality(
            1.into(),
            property(),
            Some(ClassExpression::thing())
        )
    );
    assert_eq!(
        ClassExpression::object_exact_cardinality(cardinality(), property(), None),
        ClassExpression::object_exact_cardinality(
            cardinality(),
            property(),
            Some(ClassExpression::thing())
        )
    );
    let default_range = || DataRange::datatype(iri("http://www.w3.org/2000/01/rdf-schema#Literal"));
    assert_eq!(
        ClassExpression::data_min_cardinality(0.into(), iri("urn:p"), None),
        ClassExpression::data_min_cardinality(0.into(), iri("urn:p"), Some(default_range()))
    );
    assert_eq!(
        ClassExpression::data_max_cardinality(1.into(), iri("urn:p"), None),
        ClassExpression::data_max_cardinality(1.into(), iri("urn:p"), Some(default_range()))
    );
    assert_eq!(
        ClassExpression::data_exact_cardinality(cardinality(), iri("urn:p"), None),
        ClassExpression::data_exact_cardinality(cardinality(), iri("urn:p"), Some(default_range()))
    );
}

#[test]
fn data_property_positions_preserve_order_and_repetition() {
    let left =
        ClassExpression::data_some_values_from(vec![iri("urn:p"), iri("urn:q")], range()).unwrap();
    let right =
        ClassExpression::data_some_values_from(vec![iri("urn:q"), iri("urn:p")], range()).unwrap();
    assert_ne!(left, right);
    let repeated =
        ClassExpression::data_all_values_from(vec![iri("urn:p"), iri("urn:p")], range()).unwrap();
    support::assert_round_trip(&repeated);
    assert!(ClassExpression::data_some_values_from(vec![], range()).is_err());
    assert!(ClassExpression::data_all_values_from(vec![], range()).is_err());
}

#[test]
fn cardinalities_are_unbounded_integer_values() {
    assert_eq!(Cardinality::try_new("0000").unwrap(), 0.into());
    assert_eq!(Cardinality::try_new("00042").unwrap(), 42.into());
    assert_eq!(
        cardinality().as_str(),
        "18446744073709551616000000000000000000000000000"
    );
    support::assert_round_trip(&cardinality());
    // bounded: malformed numeric categories from the nonNegativeInteger grammar.
    for invalid in ["", "-1", "+1", "1.0", " 1", "1e3", "١"] {
        assert!(Cardinality::try_new(invalid).is_err());
        let json = serde_json::to_string(invalid).unwrap();
        let bytes = postcard::to_allocvec(invalid).unwrap();
        assert!(serde_json::from_str::<Cardinality>(&json).is_err());
        assert!(postcard::from_bytes::<Cardinality>(&bytes).is_err());
        let nested = MalformedCardinality {
            cardinality: invalid,
        };
        assert!(
            serde_json::from_str::<ClassExpression>(&serde_json::to_string(&nested).unwrap())
                .is_err()
        );
        assert!(
            postcard::from_bytes::<ClassExpression>(&postcard::to_allocvec(&nested).unwrap())
                .is_err()
        );
    }
}

struct MalformedCardinality<'a> {
    cardinality: &'a str,
}
impl Serialize for MalformedCardinality<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut value =
            serializer.serialize_struct_variant("ClassExpression", 9, "ObjectMinCardinality", 3)?;
        value.serialize_field("cardinality", self.cardinality)?;
        value.serialize_field("property", &property())?;
        value.serialize_field("filler", &class())?;
        value.end()
    }
}

#[test]
fn inverses_contain_named_properties_without_recursive_inverse_shapes() {
    support::assert_round_trip(&ObjectPropertyExpression::inverse_of(iri("urn:p")));
    let invalid = r#"{"InverseOf":{"InverseOf":"urn:p"}}"#;
    assert!(serde_json::from_str::<ObjectPropertyExpression>(invalid).is_err());
}
