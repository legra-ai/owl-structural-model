//! OWL ontology identity values.

use std::fmt::{Display, Formatter};

use iri_rfc3987::Iri;

use crate::StructuralModelError;

const KIND_ONTOLOGY_ID: &str = "ontology ID";

/// OWL ontology identity: optional ontology IRI plus optional version IRI.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct OntologyId {
    ontology_iri: Option<Iri<String>>,
    version_iri: Option<Iri<String>>,
}

/// Alias emphasizing that this value is metadata rather than a document.
pub type OntologyIdentity = OntologyId;

impl OntologyId {
    /// Create an anonymous ontology ID.
    #[must_use]
    pub fn anonymous() -> Self {
        Self {
            ontology_iri: None,
            version_iri: None,
        }
    }

    /// Create a named ontology ID.
    #[must_use]
    pub fn named(ontology_iri: Iri<String>) -> Self {
        Self {
            ontology_iri: Some(ontology_iri),
            version_iri: None,
        }
    }

    /// Create a versioned ontology ID.
    #[must_use]
    pub fn versioned(ontology_iri: Iri<String>, version_iri: Iri<String>) -> Self {
        Self {
            ontology_iri: Some(ontology_iri),
            version_iri: Some(version_iri),
        }
    }

    /// Create an ontology ID from optional identity fields.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] when a version IRI is
    /// supplied without an ontology IRI.
    pub fn try_new(
        ontology_iri: Option<Iri<String>>,
        version_iri: Option<Iri<String>>,
    ) -> Result<Self, StructuralModelError> {
        if ontology_iri.is_none() && version_iri.is_some() {
            return Err(StructuralModelError::InvalidFormat {
                kind: KIND_ONTOLOGY_ID,
                value: "version-only".to_owned(),
                message: "version IRI requires ontology IRI",
            });
        }
        Ok(Self {
            ontology_iri,
            version_iri,
        })
    }

    /// Return the ontology IRI when present.
    #[must_use]
    pub fn ontology_iri(&self) -> Option<&Iri<String>> {
        self.ontology_iri.as_ref()
    }

    /// Return the version IRI when present.
    #[must_use]
    pub fn version_iri(&self) -> Option<&Iri<String>> {
        self.version_iri.as_ref()
    }

    /// Return whether this ID has no ontology IRI.
    #[must_use]
    pub fn is_anonymous(&self) -> bool {
        self.ontology_iri.is_none()
    }
}

impl Display for OntologyId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.ontology_iri, &self.version_iri) {
            (Some(ontology), Some(version)) => write!(formatter, "{ontology} @ {version}"),
            (Some(ontology), None) => formatter.write_str(ontology.as_str()),
            (None, None) => formatter.write_str("anonymous ontology"),
            (None, Some(_)) => formatter.write_str("invalid version-only ontology ID"),
        }
    }
}
