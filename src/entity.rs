//! OWL entity declarations.

use iri_rfc3987::Iri;

/// Kind of OWL entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EntityKind {
    /// OWL class.
    Class,
    /// OWL object property.
    ObjectProperty,
    /// OWL data property.
    DataProperty,
    /// OWL annotation property.
    AnnotationProperty,
    /// OWL named individual.
    NamedIndividual,
    /// OWL datatype.
    Datatype,
}

/// An OWL entity declaration target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    kind: EntityKind,
    iri: Iri<String>,
}

impl Entity {
    /// Create an entity with a concrete kind and IRI.
    #[must_use]
    pub fn new(kind: EntityKind, iri: Iri<String>) -> Self {
        Self { kind, iri }
    }

    /// Create an OWL class entity.
    #[must_use]
    pub fn class(iri: Iri<String>) -> Self { Self::new(EntityKind::Class, iri) }

    /// Create an OWL object property entity.
    #[must_use]
    pub fn object_property(iri: Iri<String>) -> Self { Self::new(EntityKind::ObjectProperty, iri) }

    /// Create an OWL data property entity.
    #[must_use]
    pub fn data_property(iri: Iri<String>) -> Self { Self::new(EntityKind::DataProperty, iri) }

    /// Create an OWL annotation property entity.
    #[must_use]
    pub fn annotation_property(iri: Iri<String>) -> Self {
        Self::new(EntityKind::AnnotationProperty, iri)
    }

    /// Create an OWL named individual entity.
    #[must_use]
    pub fn named_individual(iri: Iri<String>) -> Self {
        Self::new(EntityKind::NamedIndividual, iri)
    }

    /// Create an OWL datatype entity.
    #[must_use]
    pub fn datatype(iri: Iri<String>) -> Self { Self::new(EntityKind::Datatype, iri) }

    /// Return the entity kind.
    #[must_use]
    pub fn kind(&self) -> EntityKind { self.kind }

    /// Return the entity IRI.
    #[must_use]
    pub fn iri(&self) -> &Iri<String> { &self.iri }
}

