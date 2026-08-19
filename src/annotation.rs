//! OWL annotation values.

use iri_rfc3987::Iri;

use crate::{OwlLiteral, StructuralModelError};

const KIND_ANONYMOUS_INDIVIDUAL: &str = "anonymous individual";

/// An OWL annotation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Annotation {
    property: Iri<String>,
    value: AnnotationValue,
}

impl Annotation {
    /// Create an annotation from an annotation property IRI and value.
    #[must_use]
    pub fn new(property: Iri<String>, value: AnnotationValue) -> Self { Self { property, value } }

    /// Return the annotation property IRI.
    #[must_use]
    pub fn property(&self) -> &Iri<String> { &self.property }

    /// Return the annotation value.
    #[must_use]
    pub fn value(&self) -> &AnnotationValue { &self.value }
}

/// A value that can appear in an OWL annotation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AnnotationValue {
    /// An IRI-valued annotation.
    Iri(Iri<String>),
    /// An anonymous-individual-valued annotation.
    AnonymousIndividual(String),
    /// A literal-valued annotation.
    Literal(OwlLiteral),
}

impl AnnotationValue {
    /// Construct an anonymous individual annotation value.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::Empty`] when the label is empty.
    pub fn anonymous_individual(label: impl Into<String>) -> Result<Self, StructuralModelError> {
        let label = label.into();
        if label.is_empty() {
            return Err(StructuralModelError::Empty { kind: KIND_ANONYMOUS_INDIVIDUAL });
        }
        Ok(Self::AnonymousIndividual(label))
    }
}

/// Subject of an OWL annotation assertion.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AnnotationSubject {
    /// An IRI subject.
    Iri(Iri<String>),
    /// An anonymous individual subject.
    AnonymousIndividual(String),
}

/// Axiom annotations attached to one structural axiom.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct AxiomAnnotations {
    // bounded: annotations belong to one structural axiom.
    annotations: Vec<Annotation>,
}

impl AxiomAnnotations {
    /// Create an empty annotation set.
    #[must_use]
    pub fn empty() -> Self { Self::default() }

    /// Create annotations from a bounded axiom-local collection.
    #[must_use]
    pub fn new(annotations: Vec<Annotation>) -> Self { Self { annotations } }

    /// Return the annotations.
    #[must_use]
    pub fn as_slice(&self) -> &[Annotation] { &self.annotations }

    /// Return the annotation count.
    #[must_use]
    pub fn len(&self) -> usize { self.annotations.len() }

    /// Return whether no annotations are present.
    #[must_use]
    pub fn is_empty(&self) -> bool { self.annotations.is_empty() }
}

