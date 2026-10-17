//! OWL annotation values.

use iri_rfc3987::Iri;

use crate::{BlankNodeId, OwlLiteral};

/// An OWL annotation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Annotation<L = OwlLiteral, B = BlankNodeId> {
    property: Iri<String>,
    value: AnnotationValue<L, B>,
}

impl<L, B> Annotation<L, B> {
    /// Create an annotation from an annotation property IRI and value.
    #[must_use]
    pub fn new(property: Iri<String>, value: AnnotationValue<L, B>) -> Self {
        Self { property, value }
    }

    /// Return the annotation property IRI.
    #[must_use]
    pub fn property(&self) -> &Iri<String> {
        &self.property
    }

    /// Return the annotation value.
    #[must_use]
    pub fn value(&self) -> &AnnotationValue<L, B> {
        &self.value
    }
}

/// A value that can appear in an OWL annotation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AnnotationValue<L = OwlLiteral, B = BlankNodeId> {
    /// An IRI-valued annotation.
    Iri(Iri<String>),
    /// An anonymous-individual-valued annotation.
    AnonymousIndividual(B),
    /// A literal-valued annotation.
    Literal(L),
}

impl<L, B> AnnotationValue<L, B> {
    /// Construct an annotation value from a caller-owned anonymous identity.
    #[must_use]
    pub fn anonymous_individual(identity: B) -> Self {
        Self::AnonymousIndividual(identity)
    }
}

/// Subject of an OWL annotation assertion.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AnnotationSubject<B = BlankNodeId> {
    /// An IRI subject.
    Iri(Iri<String>),
    /// An anonymous individual subject.
    AnonymousIndividual(B),
}

/// Axiom annotations attached to one structural axiom.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct AxiomAnnotations<L = OwlLiteral, B = BlankNodeId> {
    // bounded: annotations belong to one structural axiom.
    annotations: Vec<Annotation<L, B>>,
}

impl<L, B> AxiomAnnotations<L, B> {
    /// Create an empty annotation set.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create annotations from a bounded axiom-local collection.
    #[must_use]
    pub fn new(annotations: Vec<Annotation<L, B>>) -> Self {
        Self { annotations }
    }

    /// Return the annotations.
    #[must_use]
    pub fn as_slice(&self) -> &[Annotation<L, B>] {
        &self.annotations
    }

    /// Return the annotation count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.annotations.len()
    }

    /// Return whether no annotations are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.annotations.is_empty()
    }
}

impl<L, B> Default for AxiomAnnotations<L, B> {
    fn default() -> Self {
        Self {
            annotations: Vec::new(),
        }
    }
}
