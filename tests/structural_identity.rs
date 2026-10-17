//! OWL structural identity differs from ordered Rust collection equality.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use iri_rfc3987::Iri;
use owl_structural_model::OwlLiteral;
type DataRange = owl_structural_model::DataRange;
type ClassExpression = owl_structural_model::ClassExpression;

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}

fn hash(value: &impl Hash) -> u64 {
    let mut state = DefaultHasher::new();
    value.hash(&mut state);
    state.finish()
}

#[test]
fn class_boolean_constructors_reject_singletons() {
    let operand = ClassExpression::named(iri("urn:class"));
    assert!(ClassExpression::object_union_of(vec![operand.clone()]).is_err());
    assert!(ClassExpression::object_intersection_of(vec![operand]).is_err());
}

#[test]
fn class_boolean_identity_ignores_order_and_repeated_members() {
    let first = ClassExpression::named(iri("urn:first"));
    let second = ClassExpression::named(iri("urn:second"));
    let left = ClassExpression::object_union_of(vec![first.clone(), second.clone()]).unwrap();
    let right = ClassExpression::object_union_of(vec![second, first.clone(), first]).unwrap();
    assert_eq!(left, right);
    assert_eq!(hash(&left), hash(&right));
}

#[test]
fn data_enumerations_are_sets_of_exact_literals() {
    let first = OwlLiteral::typed("005", iri("http://www.w3.org/2001/XMLSchema#integer")).unwrap();
    let second = OwlLiteral::typed("5", iri("http://www.w3.org/2001/XMLSchema#integer")).unwrap();
    assert_ne!(first, second);
    let left = DataRange::data_one_of(vec![first.clone(), second.clone()]).unwrap();
    let right = DataRange::data_one_of(vec![second, first.clone(), first]).unwrap();
    assert_eq!(left, right);
    assert_eq!(hash(&left), hash(&right));
}

#[test]
fn builtin_class_names_have_one_structural_identity() {
    assert_eq!(
        ClassExpression::named(iri("http://www.w3.org/2002/07/owl#Thing")),
        ClassExpression::thing()
    );
    assert_eq!(
        ClassExpression::named(iri("http://www.w3.org/2002/07/owl#Nothing")),
        ClassExpression::nothing()
    );
}
