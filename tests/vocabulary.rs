//! The reserved vocabulary catalogs match the OWL 2 Structural Specification.

use std::collections::HashSet;
use std::fmt::Debug;

use iri_rfc3987::Iri;
use owl_structural_model::{
    BuiltInEntity, ClassExpression, ConstrainingFacet, EntityKind, OwlDatatype,
    ReservedAnnotationProperty, VocabularyTerm,
};

fn assert_catalog<T: VocabularyTerm + Debug>(expected: usize) {
    let catalog = T::catalog();
    assert_eq!(catalog.len(), expected);
    // bounded: one entry per term of a closed catalog of at most 33 terms.
    let mut seen = HashSet::new();
    for &term in catalog {
        let iri = term.iri();
        assert_eq!(iri.as_str(), term.as_str());
        assert!(seen.insert(term.as_str()), "duplicate IRI for {term:?}");
        assert_eq!(T::from_iri(&iri), Some(term));
        assert!(term.names(&iri));
    }
}

#[test]
fn catalogs_have_the_specified_sizes_and_valid_unique_iris() {
    assert_catalog::<BuiltInEntity>(7);
    assert_catalog::<ReservedAnnotationProperty>(9);
    assert_catalog::<ConstrainingFacet>(9);
    assert_catalog::<OwlDatatype>(33);
}

#[test]
fn foreign_iris_are_not_vocabulary_terms() {
    let foreign: Iri<String> = Iri::parse("urn:example:Thing").unwrap().into();
    assert_eq!(BuiltInEntity::from_iri(&foreign), None);
    assert_eq!(OwlDatatype::from_iri(&BuiltInEntity::Literal.iri()), None);
}

#[test]
fn built_in_entities_carry_their_declaration_category() {
    assert_eq!(BuiltInEntity::Thing.entity().kind(), EntityKind::Class);
    assert_eq!(
        BuiltInEntity::BottomObjectProperty.entity().kind(),
        EntityKind::ObjectProperty
    );
    assert_eq!(
        BuiltInEntity::TopDataProperty.entity().kind(),
        EntityKind::DataProperty
    );
    assert_eq!(BuiltInEntity::Literal.entity().kind(), EntityKind::Datatype);
    assert_eq!(
        BuiltInEntity::Nothing.entity().iri(),
        &BuiltInEntity::Nothing.iri()
    );
}

#[test]
fn class_expression_helpers_use_the_vocabulary() {
    let thing: ClassExpression = ClassExpression::named(BuiltInEntity::Thing.into());
    assert!(thing.is_thing());
    assert_eq!(
        BuiltInEntity::Thing.to_string(),
        BuiltInEntity::Thing.as_str()
    );
}
