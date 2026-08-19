//! Structural ontology patch values.

use iri_rfc3987::Iri;

use crate::{Annotation, Axiom, OntologyId, StructuralModelError};

const KIND_ONTOLOGY_PATCH: &str = "ontology patch";

/// A non-empty collection of structural changes for one ontology identity.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OntologyPatch {
    target: OntologyId,
    // bounded: one submitted patch/change request, never a workspace stream.
    operations: Vec<OntologyPatchOperation>,
}

impl OntologyPatch {
    /// Create a non-empty ontology patch.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] when no operations are
    /// supplied.
    pub fn try_new(
        target: OntologyId,
        operations: Vec<OntologyPatchOperation>,
    ) -> Result<Self, StructuralModelError> {
        if operations.is_empty() {
            return Err(StructuralModelError::InvalidFormat {
                kind: KIND_ONTOLOGY_PATCH,
                value: "0 operations".to_owned(),
                message: "requires at least one operation",
            });
        }
        Ok(Self { target, operations })
    }

    /// Return the patch target ontology ID.
    #[must_use]
    pub fn target(&self) -> &OntologyId { &self.target }

    /// Return patch operations.
    #[must_use]
    pub fn operations(&self) -> &[OntologyPatchOperation] { &self.operations }
}

/// One structural ontology patch operation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OntologyPatchOperation {
    /// Add an import declaration.
    AddImport {
        /// Imported ontology IRI.
        import_iri: Iri<String>,
    },
    /// Remove an import declaration.
    RemoveImport {
        /// Imported ontology IRI.
        import_iri: Iri<String>,
    },
    /// Add an ontology annotation.
    AddAnnotation {
        /// Ontology annotation.
        annotation: Annotation,
    },
    /// Remove an ontology annotation.
    RemoveAnnotation {
        /// Ontology annotation.
        annotation: Annotation,
    },
    /// Add a structural axiom.
    AddAxiom {
        /// Structural axiom.
        axiom: Axiom,
    },
    /// Remove a structural axiom.
    RemoveAxiom {
        /// Structural axiom.
        axiom: Axiom,
    },
}
