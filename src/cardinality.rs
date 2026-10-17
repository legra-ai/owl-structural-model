//! Unbounded nonnegative cardinalities from the OWL functional grammar.

use crate::StructuralModelError;
use std::cmp::Ordering;

use serde::{Deserialize, Deserializer, Serialize};

/// An arbitrary-size nonnegative integer, independent of machine word width.
/// Leading zeroes are normalized because cardinalities are integer values,
/// unlike RDF literal lexical forms, which remain exact.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct Cardinality(String);
impl Cardinality {
    /// Parse a nonempty ASCII decimal integer without a size-dependent limit.
    /// # Errors
    /// Returns a structural error for empty, signed or nondigit input.
    pub fn try_new(value: impl Into<String>) -> Result<Self, StructuralModelError> {
        let value = value.into();
        if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(StructuralModelError::InvalidFormat {
                kind: "cardinality",
                value,
                message: "requires nonnegative ASCII decimal digits",
            });
        }
        let digits = value.trim_start_matches('0');
        Ok(Self(if digits.is_empty() {
            "0".to_owned()
        } else {
            digits.to_owned()
        }))
    }
    /// Return normalized decimal digits.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl From<u64> for Cardinality {
    fn from(value: u64) -> Self {
        Self(value.to_string())
    }
}

impl PartialOrd for Cardinality {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cardinality {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&other.0.len())
            .then_with(|| self.0.cmp(&other.0))
    }
}
impl<'de> Deserialize<'de> for Cardinality {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_new(String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
