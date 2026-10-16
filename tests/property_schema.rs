//! Property schema and keys retain OWL structural distinctions.

mod support;

use iri_rfc3987::Iri;
use owl_structural_model::{
    Annotation, AnnotationValue, Axiom, AxiomAnnotations, ClassExpression, DataRange,
    KeyProperties, ObjectPropertyExpression,
};

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}

fn property(value: &str) -> ObjectPropertyExpression {
    ObjectPropertyExpression::named(iri(value))
}

#[test]
fn object_property_relationships_preserve_operands_and_annotations() {
    let p = property("urn:parent");
    let q = ObjectPropertyExpression::inverse_of(property("urn:child"));
    let equivalent = Axiom::equivalent_object_properties(vec![p.clone(), q.clone()]).unwrap();
    assert!(
        matches!(&equivalent, Axiom::EquivalentObjectProperties { properties, .. }
        if properties == &[p.clone(), q.clone()])
    );
    let disjoint = Axiom::disjoint_object_properties(vec![p.clone(), q.clone()]).unwrap();
    assert!(
        matches!(&disjoint, Axiom::DisjointObjectProperties { properties, .. }
        if properties == &[p.clone(), q.clone()])
    );
    let inverse = Axiom::inverse_object_properties(p.clone(), q.clone());
    assert!(
        matches!(&inverse, Axiom::InverseObjectProperties { first, second, .. }
        if first == &p && second == &q)
    );
    let range = Axiom::object_property_range(p.clone(), ClassExpression::Thing);
    assert!(
        matches!(&range, Axiom::ObjectPropertyRange { property, range: ClassExpression::Thing, .. }
        if property == &p)
    );
    let domain = Axiom::object_property_domain(p.clone(), ClassExpression::Nothing);
    assert!(
        matches!(&domain, Axiom::ObjectPropertyDomain { property, domain: ClassExpression::Nothing, .. }
        if property == &p)
    );
    // bounded: five structural schema fixtures.
    for axiom in [equivalent, disjoint, inverse, range, domain] {
        assert_annotated_round_trip(axiom);
    }
}

#[test]
fn data_property_relationships_and_datatype_definitions_preserve_ranges() {
    let p = iri("urn:age");
    let q = iri("urn:years");
    let sub = Axiom::sub_data_property_of(p.clone(), q.clone());
    assert!(
        matches!(&sub, Axiom::SubDataPropertyOf { sub_property, super_property, .. }
        if sub_property == &p && super_property == &q)
    );
    let equivalent = Axiom::equivalent_data_properties(vec![p.clone(), q.clone()]).unwrap();
    assert!(
        matches!(&equivalent, Axiom::EquivalentDataProperties { properties, .. }
        if properties == &[p.clone(), q.clone()])
    );
    let disjoint = Axiom::disjoint_data_properties(vec![p.clone(), q.clone()]).unwrap();
    assert!(
        matches!(&disjoint, Axiom::DisjointDataProperties { properties, .. }
        if properties == &[p.clone(), q])
    );
    let domain = Axiom::data_property_domain(p.clone(), ClassExpression::Thing);
    assert!(
        matches!(&domain, Axiom::DataPropertyDomain { property, domain: ClassExpression::Thing, .. }
        if property == &p)
    );
    let range = DataRange::data_complement_of(DataRange::datatype(iri("urn:negative")));
    let range_axiom = Axiom::data_property_range(p.clone(), range.clone());
    assert!(
        matches!(&range_axiom, Axiom::DataPropertyRange { property, range: actual, .. }
        if property == &p && actual == &range)
    );
    let definition = Axiom::datatype_definition(iri("urn:positive"), range.clone());
    assert!(
        matches!(&definition, Axiom::DatatypeDefinition { datatype, range: actual, .. }
        if datatype == &iri("urn:positive") && actual == &range)
    );
    // bounded: six structural schema fixtures.
    for axiom in [sub, equivalent, disjoint, domain, range_axiom, definition] {
        assert_annotated_round_trip(axiom);
    }
}

#[test]
fn equivalent_and_disjoint_properties_require_two_operands() {
    // bounded: zero and one are the invalid arities for n-ary property axioms.
    for objects in [vec![], vec![property("urn:p")]] {
        assert!(Axiom::equivalent_object_properties(objects.clone()).is_err());
        assert!(Axiom::disjoint_object_properties(objects.clone()).is_err());
        for kind in ["EquivalentObjectProperties", "DisjointObjectProperties"] {
            assert_bad_properties(kind, &objects);
        }
    }
    // bounded: zero and one are the invalid arities for n-ary property axioms.
    for data in [vec![], vec![iri("urn:p")]] {
        assert!(Axiom::equivalent_data_properties(data.clone()).is_err());
        assert!(Axiom::disjoint_data_properties(data.clone()).is_err());
        for kind in ["EquivalentDataProperties", "DisjointDataProperties"] {
            assert_bad_properties(kind, &data);
        }
    }
}

fn assert_bad_properties<T: serde::Serialize>(kind: &str, properties: &[T]) {
    let properties = serde_json::to_string(properties).unwrap();
    let encoded = format!(
        r#"{{"{kind}":{{"properties":{properties},"annotations":{{"annotations":[]}}}}}}"#,
    );
    let error = serde_json::from_str::<Axiom>(&encoded).unwrap_err();
    assert!(error.to_string().contains("requires at least two"));
}

#[test]
fn keys_keep_object_and_data_properties_separate() {
    let p = ObjectPropertyExpression::inverse_of(property("urn:parent"));
    let d = iri("urn:identifier");
    // bounded: object-only, data-only, and mixed key fixtures.
    for (objects, data) in [
        (vec![p.clone()], vec![]),
        (vec![], vec![d.clone()]),
        (vec![p.clone()], vec![d.clone()]),
    ] {
        let properties = KeyProperties::try_new(objects.clone(), data.clone()).unwrap();
        assert_eq!(properties.object_properties(), objects);
        assert_eq!(properties.data_properties(), data);
        let class = ClassExpression::object_complement_of(ClassExpression::Nothing);
        let axiom = Axiom::has_key(class.clone(), properties.clone());
        assert!(
            matches!(&axiom, Axiom::HasKey { class: actual, properties: actual_properties, .. }
            if actual == &class && actual_properties == &properties)
        );
        assert_annotated_round_trip(axiom);
    }
}

#[test]
fn empty_keys_fail_at_construction_and_deserialization() {
    assert!(KeyProperties::try_new(vec![], vec![]).is_err());
    let error =
        serde_json::from_str::<KeyProperties>(r#"{"object_properties":[],"data_properties":[]}"#)
            .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("requires at least one key property")
    );
    let error = serde_json::from_str::<Axiom>(
        r#"{"HasKey":{"class":"Thing","properties":{"object_properties":[],"data_properties":[]},"annotations":{"annotations":[]}}}"#,
    ).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("requires at least one key property")
    );
}

fn assert_annotated_round_trip(axiom: Axiom) {
    assert!(axiom.annotations().is_empty());
    let annotations = AxiomAnnotations::new(vec![Annotation::new(
        iri("urn:source"),
        AnnotationValue::Iri(iri("urn:fixture")),
    )]);
    let annotated = axiom.with_annotations(annotations.clone());
    assert_eq!(annotated.annotations(), &annotations);
    support::assert_round_trip(&annotated);
}
