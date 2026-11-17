//! Shared behavior of every reserved vocabulary term.

use iri_rfc3987::Iri;

/// A term from a closed catalog of standard OWL 2, RDF, RDFS or XSD IRIs.
pub trait VocabularyTerm: Copy + Eq + 'static {
    /// Return every term of this catalog, in specification order.
    fn catalog() -> &'static [Self];

    /// Return the full IRI text of this term.
    fn as_str(self) -> &'static str;

    /// Return this term as an owned IRI.
    ///
    /// # Panics
    ///
    /// Never for the crate's catalogs: every term is a valid absolute IRI, which
    /// the vocabulary test suite verifies for each catalog entry.
    #[must_use]
    fn iri(self) -> Iri<String> {
        Iri::parse(self.as_str())
            .expect("standard vocabulary IRI is valid")
            .into()
    }

    /// Return whether `iri` names exactly this term.
    #[must_use]
    fn names(self, iri: &Iri<String>) -> bool {
        iri.as_str() == self.as_str()
    }

    /// Find the catalog term named exactly by `iri`.
    #[must_use]
    fn from_iri(iri: &Iri<String>) -> Option<Self> {
        Self::catalog().iter().copied().find(|term| term.names(iri))
    }
}
