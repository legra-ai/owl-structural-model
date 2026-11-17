//! Generators for IRIs, literals, individuals and annotations.

use iri_rfc3987::Iri;
use owl_structural_model::{
    BuiltInEntity, Cardinality, ConstrainingFacet, Individual, ObjectPropertyExpression,
    OwlDatatype, OwlLiteral, ReservedAnnotationProperty, VocabularyTerm,
};
use proptest::prelude::*;
use proptest::sample::select;
use rdf_structural_model::{BlankNodeId, LanguageTag};

pub type Annotation = owl_structural_model::Annotation;
pub type AnnotationValue = owl_structural_model::AnnotationValue;
pub type AxiomAnnotations = owl_structural_model::AxiomAnnotations;

/// The largest operand count generated for any n-ary constructor.
pub const MAX_OPERANDS: usize = 4;

pub fn iri() -> impl Strategy<Value = Iri<String>> {
    "[a-z][a-z0-9]{0,5}".prop_map(|local| {
        Iri::parse(format!("urn:x:{local}").as_str())
            .unwrap()
            .into()
    })
}

pub fn datatype_iri() -> impl Strategy<Value = Iri<String>> {
    prop_oneof![
        select(OwlDatatype::ALL.as_slice()).prop_map(OwlDatatype::iri),
        Just(BuiltInEntity::Literal.iri()),
        iri(),
    ]
}

pub fn facet_iri() -> impl Strategy<Value = Iri<String>> {
    select(ConstrainingFacet::ALL.as_slice()).prop_map(ConstrainingFacet::iri)
}

pub fn annotation_property() -> impl Strategy<Value = Iri<String>> {
    prop_oneof![
        select(ReservedAnnotationProperty::ALL.as_slice())
            .prop_map(ReservedAnnotationProperty::iri),
        iri(),
    ]
}

pub fn blank() -> impl Strategy<Value = BlankNodeId> {
    (iri(), "b[a-z0-9]{0,4}").prop_map(|(scope, label)| BlankNodeId::try_new(scope, label).unwrap())
}

pub fn literal() -> impl Strategy<Value = OwlLiteral> {
    prop_oneof![
        "[ -~]{0,8}".prop_map(OwlLiteral::string),
        ("[0-9]{1,4}", select(OwlDatatype::ALL.as_slice()))
            .prop_map(|(lexical, datatype)| OwlLiteral::typed(lexical, datatype.iri()).unwrap()),
        (
            "[a-z]{0,6}",
            select(["en", "fr", "de-CH", "nl-BE"].as_slice())
        )
            .prop_map(|(lexical, tag)| {
                OwlLiteral::language_tagged(lexical, LanguageTag::try_new(tag.to_owned()).unwrap())
            }),
    ]
}

pub fn individual() -> impl Strategy<Value = Individual> {
    prop_oneof![
        iri().prop_map(Individual::named),
        blank().prop_map(Individual::anonymous),
    ]
}

pub fn object_property() -> impl Strategy<Value = ObjectPropertyExpression> {
    prop_oneof![
        iri().prop_map(ObjectPropertyExpression::named),
        iri().prop_map(ObjectPropertyExpression::inverse_of),
    ]
}

pub fn cardinality() -> impl Strategy<Value = Cardinality> {
    prop_oneof![
        any::<u64>().prop_map(Cardinality::from),
        "[0-9]{1,30}".prop_map(|digits| Cardinality::try_new(digits).unwrap()),
    ]
}

pub fn annotation_value() -> impl Strategy<Value = AnnotationValue> {
    prop_oneof![
        iri().prop_map(AnnotationValue::Iri),
        literal().prop_map(AnnotationValue::Literal),
        blank().prop_map(AnnotationValue::anonymous_individual),
    ]
}

/// Annotations nested at most two levels deep.
pub fn annotations() -> impl Strategy<Value = AxiomAnnotations> {
    let flat = (annotation_property(), annotation_value())
        .prop_map(|(property, value)| Annotation::new(property, value));
    let nested = (
        annotation_property(),
        annotation_value(),
        prop::collection::vec(flat, 0..2),
    )
        .prop_map(|(property, value, inner)| {
            Annotation::new(property, value).with_annotations(AxiomAnnotations::new(inner))
        });
    prop::collection::vec(nested, 0..3).prop_map(AxiomAnnotations::new)
}
