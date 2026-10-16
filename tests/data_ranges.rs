//! OWL 2 data ranges retain structure without evaluating datatype values.

use iri_rfc3987::Iri;
use owl_structural_model::{DataRange, FacetRestriction, OwlLiteral};

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}

fn datatype(name: &str) -> DataRange {
    DataRange::datatype(iri(&format!("http://www.w3.org/2001/XMLSchema#{name}")))
}

#[test]
fn every_data_range_form_round_trips_with_exact_literals() {
    let integer = iri("http://www.w3.org/2001/XMLSchema#integer");
    let five = OwlLiteral::typed("005", integer.clone());
    let min = FacetRestriction::new(
        iri("http://www.w3.org/2001/XMLSchema#minInclusive"),
        five.clone(),
    );
    assert_eq!(min.value(), &five);
    assert_eq!(
        min.facet(),
        &iri("http://www.w3.org/2001/XMLSchema#minInclusive")
    );
    let restricted = DataRange::datatype_restriction(integer, vec![min]).unwrap();
    // bounded: one example of each of the six OWL 2 data-range forms.
    let forms = [
        datatype("integer"),
        DataRange::data_intersection_of(vec![datatype("integer"), restricted.clone()]).unwrap(),
        DataRange::data_union_of(vec![datatype("string"), datatype("integer")]).unwrap(),
        DataRange::data_complement_of(restricted.clone()),
        DataRange::data_one_of(vec![
            five.clone(),
            OwlLiteral::language_tagged("cinq", "fr").unwrap(),
        ])
        .unwrap(),
        restricted,
    ];
    for range in forms {
        let encoded = serde_json::to_string(&range).unwrap();
        assert_eq!(serde_json::from_str::<DataRange>(&encoded).unwrap(), range);
    }
    assert_ne!(
        five,
        OwlLiteral::typed("5", iri("http://www.w3.org/2001/XMLSchema#integer"))
    );
}

#[test]
fn empty_and_singleton_boolean_data_ranges_are_rejected() {
    // bounded: the two invalid arities for intersection and union.
    for operands in [vec![], vec![datatype("integer")]] {
        assert!(DataRange::data_intersection_of(operands.clone()).is_err());
        assert!(DataRange::data_union_of(operands.clone()).is_err());
        let encoded = serde_json::to_string(&operands).unwrap();
        for kind in ["DataIntersectionOf", "DataUnionOf"] {
            let input = format!(r#"{{"{kind}":{encoded}}}"#);
            let error = serde_json::from_str::<DataRange>(&input).unwrap_err();
            assert!(error.to_string().contains("requires at least two"));
        }
    }
}

#[test]
fn empty_enumerations_and_facet_lists_are_rejected() {
    assert!(DataRange::data_one_of(vec![]).is_err());
    assert!(DataRange::datatype_restriction(iri("urn:datatype"), vec![]).is_err());
    let enumeration = serde_json::from_str::<DataRange>(r#"{"DataOneOf":[]}"#).unwrap_err();
    assert!(enumeration.to_string().contains("requires at least one"));
    let restriction = serde_json::from_str::<DataRange>(
        r#"{"DatatypeRestriction":{"datatype":"urn:datatype","restrictions":[]}}"#,
    )
    .unwrap_err();
    assert!(restriction.to_string().contains("requires at least one"));
}

#[test]
fn nested_deserialization_cannot_bypass_arity_checks() {
    let input = r#"{"DataComplementOf":{"DataUnionOf":[{"Datatype":"urn:datatype"}]}}"#;
    let error = serde_json::from_str::<DataRange>(input).unwrap_err();
    assert!(error.to_string().contains("requires at least two"));
}
