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
        #[serde(deserialize_with = "OwlLiteral::deserialize_language")]
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
        Self::require_language(&language)?;
        Ok(Self::LanguageTagged {
            lexical: lexical.into(),
            language,
        })
    }
    fn require_language(language: &str) -> Result<(), StructuralModelError> {
        if language.is_empty() {
            return Err(StructuralModelError::Empty {
                kind: KIND_LANGUAGE_TAG,
            });
        }
        Ok(())
    }

    fn deserialize_language<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<String, D::Error> {
        let language = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::require_language(&language).map_err(serde::de::Error::custom)?;
        Ok(language)
    }
}
