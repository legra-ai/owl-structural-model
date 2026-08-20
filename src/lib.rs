#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

//! Typed OWL 2 structural values without an RDF-store dependency.

mod annotation;
mod axiom;
mod class_expression;
mod entity;
mod error;
mod literal;
mod ontology;
mod patch;

pub use annotation::{Annotation, AnnotationSubject, AnnotationValue, AxiomAnnotations};
pub use axiom::Axiom;
pub use class_expression::{ClassExpression, ObjectPropertyExpression};
pub use entity::{Entity, EntityKind};
pub use error::StructuralModelError;
pub use literal::OwlLiteral;
pub use ontology::{OntologyId, OntologyIdentity};
pub use patch::{OntologyPatch, OntologyPatchOperation};

#[cfg(test)]
mod tests {
    use iri_rfc3987::Iri;

    use super::*;

    fn iri(value: &str) -> Iri<String> {
        Iri::parse(value).expect("test IRI is valid").into()
    }

    #[test]
    fn constructors_preserve_structural_shape() {
        let axiom = Axiom::sub_class_of(
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

        let error = OwlLiteral::language_tagged("hello", "").unwrap_err();
        assert!(matches!(error, StructuralModelError::Empty { .. }));
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
    /// let axiom = Axiom::sub_class_of(
    ///     ClassExpression::named(parse("https://example.org/Customer")),
    ///     ClassExpression::named(parse("https://example.org/Party")),
    /// );
    /// assert!(matches!(axiom, Axiom::SubClassOf { .. }));
    /// ```
    #[test]
    fn doctest_anchor() {}
}
