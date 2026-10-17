//! Structural ontology patch values.

use iri_rfc3987::Iri;

use crate::{Annotation, Axiom, OntologyId, StructuralModelError};

const KIND_ONTOLOGY_PATCH: &str = "ontology patch";

/// A non-empty collection of structural changes for one ontology identity.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(
    try_from = "OntologyPatchInput<L, B>",
    bound(deserialize = "L: serde::Deserialize<'de>, B: serde::Deserialize<'de>")
)]
pub struct OntologyPatch<L = crate::OwlLiteral, B = crate::BlankNodeId> {
    target: OntologyId,
    // bounded: one submitted patch/change request, never a workspace stream.
    operations: Vec<OntologyPatchOperation<L, B>>,
}

impl<L, B> OntologyPatch<L, B> {
    /// Create a non-empty ontology patch.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] when no operations are
    /// supplied.
    pub fn try_new(
        target: OntologyId,
        operations: Vec<OntologyPatchOperation<L, B>>,
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
    pub fn target(&self) -> &OntologyId {
        &self.target
    }

    /// Return patch operations.
    #[must_use]
    pub fn operations(&self) -> &[OntologyPatchOperation<L, B>] {
        &self.operations
    }
}

/// One structural ontology patch operation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum OntologyPatchOperation<L = crate::OwlLiteral, B = crate::BlankNodeId> {
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
        annotation: Annotation<L, B>,
    },
    /// Remove an ontology annotation.
    RemoveAnnotation {
        /// Ontology annotation.
        annotation: Annotation<L, B>,
    },
    /// Add a structural axiom.
    AddAxiom {
        /// Structural axiom.
        axiom: Axiom<L, B>,
    },
    /// Remove a structural axiom.
    RemoveAxiom {
        /// Structural axiom.
        axiom: Axiom<L, B>,
    },
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename = "OntologyPatch<L, B>")]
struct OntologyPatchInput<L, B> {
    target: OntologyId,
    // bounded: operations of one submitted structural patch.
    operations: Vec<OntologyPatchOperation<L, B>>,
}

impl<L, B> TryFrom<OntologyPatchInput<L, B>> for OntologyPatch<L, B> {
    type Error = StructuralModelError;

    fn try_from(input: OntologyPatchInput<L, B>) -> Result<Self, Self::Error> {
        Self::try_new(input.target, input.operations)
    }
}
