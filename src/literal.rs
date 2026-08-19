//! OWL literal values.

use iri_rfc3987::Iri;

use crate::StructuralModelError;

const KIND_LANGUAGE_TAG: &str = "language tag";

/// A literal value used in OWL structural annotations and axioms.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum OwlLiteral {
    /// A typed literal.
    Typed {
        /// Literal lexical form.
        lexical: String,
        /// Literal datatype IRI.
        datatype: Iri<String>,
    },
    /// A language-tagged string literal.
    LanguageTagged {
        /// Literal lexical form.
        lexical: String,
        /// BCP-47 language tag.
        language: String,
    },
}

impl OwlLiteral {
    /// Create a typed literal.
    #[must_use]
    pub fn typed(lexical: impl Into<String>, datatype: Iri<String>) -> Self {
        Self::Typed {
            lexical: lexical.into(),
            datatype,
        }
    }

    /// Create a language-tagged literal.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::Empty`] when the language tag is empty.
    pub fn language_tagged(
        lexical: impl Into<String>,
        language: impl Into<String>,
    ) -> Result<Self, StructuralModelError> {
        let language = language.into();
        if language.is_empty() {
            return Err(StructuralModelError::Empty {
                kind: KIND_LANGUAGE_TAG,
            });
        }
        Ok(Self::LanguageTagged {
            lexical: lexical.into(),
            language,
        })
    }
}
