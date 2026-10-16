//! Structural property axioms shared by editors and reasoner adapters.

mod support;

use iri_rfc3987::Iri;
use owl_structural_model::{
    Annotation, AnnotationValue, Axiom, AxiomAnnotations, ObjectPropertyChain,
    ObjectPropertyExpression, StructuralModelError, SubObjectPropertyExpression,
};

fn iri(value: &str) -> Iri<String> {
    Iri::parse(value).unwrap().into()
}

fn property(value: &str) -> ObjectPropertyExpression {
    ObjectPropertyExpression::named(iri(value))
}

#[test]
fn property_characteristics_keep_distinct_axiom_kinds_and_annotations() {
    let p = ObjectPropertyExpression::inverse_of(property("urn:parent"));
    // bounded: one instance of each supported object-property characteristic.
    let axioms = [
        Axiom::functional_object_property(p.clone()),
        Axiom::inverse_functional_object_property(p.clone()),
        Axiom::reflexive_object_property(p.clone()),
        Axiom::irreflexive_object_property(p.clone()),
        Axiom::symmetric_object_property(p.clone()),
        Axiom::asymmetric_object_property(p.clone()),
        Axiom::transitive_object_property(p.clone()),
    ];
    let annotations = AxiomAnnotations::new(vec![Annotation::new(
        iri("urn:source"),
        AnnotationValue::Iri(iri("urn:fixture")),
    )]);
    for (index, axiom) in axioms.into_iter().enumerate() {
        assert!(axiom.annotations().is_empty());
        let annotated = axiom.with_annotations(annotations.clone());
        assert_eq!(annotated.annotations(), &annotations);
        match (index, &annotated) {
            (0, Axiom::FunctionalObjectProperty { property, .. })
            | (1, Axiom::InverseFunctionalObjectProperty { property, .. })
            | (2, Axiom::ReflexiveObjectProperty { property, .. })
            | (3, Axiom::IrreflexiveObjectProperty { property, .. })
            | (4, Axiom::SymmetricObjectProperty { property, .. })
            | (5, Axiom::AsymmetricObjectProperty { property, .. })
            | (6, Axiom::TransitiveObjectProperty { property, .. }) => assert_eq!(property, &p),
            _ => panic!("characteristic constructor changed the axiom kind"),
        }
        support::assert_round_trip(&annotated);
    }
}

#[test]
fn functional_data_property_uses_a_data_property_iri() {
    let axiom = Axiom::functional_data_property(iri("urn:age"));
    assert!(
        matches!(axiom, Axiom::FunctionalDataProperty { ref property, .. }
        if property == &iri("urn:age"))
    );
    support::assert_round_trip(&axiom);
}

#[test]
fn property_chain_preserves_order_repetitions_and_inverses() {
    let parent = property("urn:parent");
    let inverse = ObjectPropertyExpression::inverse_of(property("urn:sibling"));
    // bounded: three members of one property-chain fixture.
    let members = vec![parent.clone(), inverse, parent.clone()];
    let chain = ObjectPropertyChain::try_new(members.clone()).unwrap();
    assert_eq!(chain.as_slice(), members);
    let reversed =
        ObjectPropertyChain::try_new(vec![members[1].clone(), parent.clone(), parent]).unwrap();
    assert_ne!(chain, reversed);
    let axiom = Axiom::sub_object_property_of(
        SubObjectPropertyExpression::Chain(chain),
        property("urn:relative"),
    );
    support::assert_round_trip(&axiom);
}

#[test]
fn short_chains_are_rejected_at_construction_and_deserialization() {
    // bounded: the two invalid chain lengths, zero and one.
    for members in [vec![], vec![property("urn:parent")]] {
        assert!(matches!(
            ObjectPropertyChain::try_new(members.clone()),
            Err(StructuralModelError::InvalidFormat { .. })
        ));
        let encoded = serde_json::to_string(&members).unwrap();
        let error = serde_json::from_str::<ObjectPropertyChain>(&encoded).unwrap_err();
        assert!(error.to_string().contains("requires at least two"));
        let encoded_axiom = format!(
            r#"{{"SubObjectPropertyOf":{{"sub_property":{{"Chain":{encoded}}},"super_property":{{"Named":"urn:relative"}},"annotations":{{"annotations":[]}}}}}}"#,
        );
        let error = serde_json::from_str::<Axiom>(&encoded_axiom).unwrap_err();
        assert!(error.to_string().contains("requires at least two"));
    }
}

#[test]
fn two_member_chain_and_single_subproperty_are_distinct() {
    let p = property("urn:parent");
    let chain = ObjectPropertyChain::try_new(vec![p.clone(), p.clone()]).unwrap();
    assert_eq!(chain.as_slice(), &[p.clone(), p.clone()]);
    let simple =
        Axiom::sub_object_property_of(SubObjectPropertyExpression::Property(p.clone()), p.clone());
    let complex = Axiom::sub_object_property_of(SubObjectPropertyExpression::Chain(chain), p);
    assert_ne!(simple, complex);
}
