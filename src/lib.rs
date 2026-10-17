#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

//! Typed OWL 2 structural values without an RDF-store dependency.

mod annotation;
mod annotation_axiom;
mod assertion;
mod axiom;
mod axiom_annotations;
mod cardinality;
mod class_axiom;
mod class_boolean;
mod class_expression;
mod class_restriction;
mod data_axiom;
mod data_range;
mod entity;
mod error;
mod individual;
mod key;
mod kind;
mod ontology;
mod operand_set;
mod operands;
mod patch;
mod property_axiom;
mod property_chain;
mod property_schema;

pub use annotation::{Annotation, AnnotationSubject, AnnotationValue, AxiomAnnotations};
pub use axiom::Axiom;
pub use cardinality::Cardinality;
pub use class_expression::{ClassExpression, ObjectPropertyExpression};
pub use data_range::{DataRange, FacetRestriction};
pub use entity::{Entity, EntityKind};
pub use error::StructuralModelError;
pub use individual::Individual;
pub use key::KeyProperties;
pub use kind::{AxiomKind, ClassExpressionKind, DataRangeKind};
pub use ontology::{OntologyId, OntologyIdentity};
pub use operand_set::{OperandSet, UnorderedPair};
pub use operands::OperandList;
pub use patch::{OntologyPatch, OntologyPatchOperation};
pub use property_chain::{ObjectPropertyChain, SubObjectPropertyExpression};
pub use rdf_structural_model::{BlankNodeId, Literal as OwlLiteral};

#[cfg(test)]
mod tests {
    use iri_rfc3987::Iri;

    use super::*;
    type ClassExpression = super::ClassExpression;

    fn iri(value: &str) -> Iri<String> {
        Iri::parse(value).expect("test IRI is valid").into()
    }

    #[test]
    fn constructors_preserve_structural_shape() {
        let axiom: Axiom = Axiom::sub_class_of(
            ClassExpression::named(iri("https://example.org/Customer")),
            ClassExpression::named(iri("https://example.org/Party")),
        );

        assert!(matches!(axiom, Axiom::SubClassOf { .. }));
        assert_eq!(
            Entity::class(iri("https://example.org/Customer")).kind(),
            EntityKind::Class
        );
    }

    #[test]
    fn invalid_structural_values_fail_at_construction() {
        let error = ClassExpression::object_union_of(Vec::new()).unwrap_err();
        assert!(matches!(error, StructuralModelError::InvalidFormat { .. }));

        assert!(rdf_structural_model::LanguageTag::try_new("").is_err());
    }

    #[test]
    fn ontology_identity_rejects_version_without_name() {
        let error = OntologyId::try_new(None, Some(iri("https://example.org/v1"))).unwrap_err();
        assert!(matches!(error, StructuralModelError::InvalidFormat { .. }));
    }

    /// ```
    /// use iri_rfc3987::Iri;
    /// use owl_structural_model::{Axiom, ClassExpression};
    ///
    /// let parse = |value: &str| Iri::parse(value).expect("valid IRI").into();
    /// let axiom: Axiom = Axiom::sub_class_of(
    ///     ClassExpression::named(parse("https://example.org/Customer")),
    ///     ClassExpression::named(parse("https://example.org/Party")),
    /// );
    /// assert!(matches!(axiom, Axiom::SubClassOf { .. }));
    /// ```
    #[test]
    fn doctest_anchor() {}
}
