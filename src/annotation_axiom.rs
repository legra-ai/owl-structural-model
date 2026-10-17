//! Checked construction for annotation axiom families.
use crate::{AnnotationSubject, AnnotationValue, Axiom, AxiomAnnotations};
use iri_rfc3987::Iri;
impl<L, B> Axiom<L, B> {
    /// Construct `AnnotationAssertion` without axiom annotations.
    #[must_use]
    pub fn annotation_assertion(
        property: Iri<String>,
        subject: AnnotationSubject<B>,
        value: AnnotationValue<L, B>,
    ) -> Self {
        Self::AnnotationAssertion {
            property,
            subject,
            value,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `SubAnnotationPropertyOf` without axiom annotations.
    #[must_use]
    pub fn sub_annotation_property_of(
        sub_property: Iri<String>,
        super_property: Iri<String>,
    ) -> Self {
        Self::SubAnnotationPropertyOf {
            sub_property,
            super_property,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `AnnotationPropertyDomain` without axiom annotations.
    #[must_use]
    pub fn annotation_property_domain(property: Iri<String>, domain: Iri<String>) -> Self {
        Self::AnnotationPropertyDomain {
            property,
            domain,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `AnnotationPropertyRange` without axiom annotations.
    #[must_use]
    pub fn annotation_property_range(property: Iri<String>, range: Iri<String>) -> Self {
        Self::AnnotationPropertyRange {
            property,
            range,
            annotations: AxiomAnnotations::empty(),
        }
    }
}
