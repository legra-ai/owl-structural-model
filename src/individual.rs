//! Named and scoped anonymous individuals in assertions and restrictions.

use crate::BlankNodeId;
use iri_rfc3987::Iri;

/// An OWL individual with a caller-selected anonymous identity representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Individual<B = BlankNodeId> {
    /// A named individual.
    Named(Iri<String>),
    /// An anonymous individual, scoped by the default RDF identity type.
    Anonymous(B),
}
impl<B> Individual<B> {
    /// Create a named individual.
    #[must_use]
    pub fn named(iri: Iri<String>) -> Self {
        Self::Named(iri)
    }
    /// Create an anonymous individual using the caller's identity contract.
    #[must_use]
    pub fn anonymous(identity: B) -> Self {
        Self::Anonymous(identity)
    }
}
