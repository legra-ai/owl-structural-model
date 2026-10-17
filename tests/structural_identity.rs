//! OWL structural identity differs from ordered Rust collection equality.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use iri_rfc3987::Iri;
use owl_structural_model::OwlLiteral;
use owl_structural_model::{Individual, KeyProperties, ObjectPropertyExpression};
use rdf_structural_model::BlankNodeId;
type DataRange = owl_structural_model::DataRange;
type ClassExpression = owl_structural_model::ClassExpression;
type Axiom = owl_structural_model::Axiom;
type Annotation = owl_structural_model::Annotation;
type AnnotationValue = owl_structural_model::AnnotationValue;
type AxiomAnnotations = owl_structural_model::AxiomAnnotations;

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
    assert!(ClassExpression::thing().is_thing());
    assert!(ClassExpression::nothing().is_nothing());
    assert!(!ClassExpression::nothing().is_thing());
    assert!(!ClassExpression::thing().is_nothing());
}

#[test]
fn unordered_axiom_operands_and_keys_have_consistent_equality_and_hashing() {
    let first = ClassExpression::named(iri("urn:first"));
    let second = ClassExpression::named(iri("urn:second"));
    let left = Axiom::equivalent_classes(vec![first.clone(), second.clone()]).unwrap();
    let right = Axiom::equivalent_classes(vec![second, first.clone(), first]).unwrap();
    assert_eq!(left, right);
    assert_eq!(hash(&left), hash(&right));
    let p = ObjectPropertyExpression::named(iri("urn:p"));
    let q = ObjectPropertyExpression::named(iri("urn:q"));
    let left = KeyProperties::try_new(vec![p.clone(), q.clone()], vec![iri("urn:data")]).unwrap();
    let right = KeyProperties::try_new(
        vec![q.clone(), p.clone(), p.clone()],
        vec![iri("urn:data"), iri("urn:data")],
    )
    .unwrap();
    assert_eq!(left, right);
    assert_eq!(hash(&left), hash(&right));
    let left = Axiom::inverse_object_properties(p.clone(), q.clone());
    let right = Axiom::inverse_object_properties(q, p);
    assert_eq!(left, right);
    assert_eq!(hash(&left), hash(&right));
}

#[test]
fn recursive_annotation_identity_is_unordered_but_preserves_meaning() {
    let first = Annotation::new(
        iri("urn:first"),
        AnnotationValue::Literal(OwlLiteral::string("first")),
    );
    let second = Annotation::new(
        iri("urn:second"),
        AnnotationValue::Literal(OwlLiteral::string("second")),
    );
    let left = Annotation::new(iri("urn:outer"), AnnotationValue::Iri(iri("urn:value")))
        .with_annotations(AxiomAnnotations::new(vec![first.clone(), second.clone()]));
    let right = Annotation::new(iri("urn:outer"), AnnotationValue::Iri(iri("urn:value")))
        .with_annotations(AxiomAnnotations::new(vec![second, first.clone(), first]));
    assert_eq!(left, right);
    assert_eq!(hash(&left), hash(&right));
    let different = Annotation::new(iri("urn:outer"), AnnotationValue::Iri(iri("urn:value")));
    assert_ne!(left, different);
}

#[test]
fn individual_sets_ignore_order_but_assertion_positions_do_not() {
    let named = Individual::named(iri("urn:named"));
    let anonymous =
        Individual::anonymous(BlankNodeId::try_new(iri("urn:scope"), "b0".to_owned()).unwrap());
    let left = Axiom::same_individual(vec![named.clone(), anonymous.clone()]).unwrap();
    let right =
        Axiom::same_individual(vec![anonymous.clone(), named.clone(), named.clone()]).unwrap();
    assert_eq!(left, right);
    assert_eq!(hash(&left), hash(&right));
    let property = ObjectPropertyExpression::named(iri("urn:p"));
    let forward =
        Axiom::object_property_assertion(property.clone(), named.clone(), anonymous.clone());
    let reverse = Axiom::object_property_assertion(property, anonymous, named);
    assert_ne!(forward, reverse);
}
