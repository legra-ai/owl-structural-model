//! Arity-checked source occurrences in one structural value.

use crate::StructuralModelError;
use serde::{Deserialize, Deserializer, Serialize};
use std::ops::Deref;

/// An ordered structural association with a minimum occurrence count.
/// Construction needs no traits on the supplied payload type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct OperandList<T, const MIN: usize> {
    // bounded: operands of one caller-budgeted structural value, never a graph.
    items: Vec<T>,
}
impl<T, const MIN: usize> OperandList<T, MIN> {
    /// Check the minimum occurrence count without interpreting operands.
    /// # Errors
    /// Returns a structural error when fewer than `MIN` occurrences are supplied.
    pub fn try_new(items: Vec<T>) -> Result<Self, StructuralModelError> {
        if items.len() < MIN {
            return Err(StructuralModelError::InvalidFormat {
                kind: "structural operands",
                value: items.len().to_string(),
                message: match MIN {
                    1 => "requires at least one operand",
                    2 => "requires at least two operands",
                    _ => "too few operands for the structural minimum",
                },
            });
        }
        Ok(Self { items })
    }
    /// Return source occurrences in their supplied order.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }
    /// Consume this structural association.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}
impl<T> OperandList<T, 0> {
    /// Construct an unrestricted, possibly empty association.
    #[must_use]
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }
}
impl<T, const MIN: usize> Deref for OperandList<T, MIN> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<'a, T, const MIN: usize> IntoIterator for &'a OperandList<T, MIN> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}
impl<'de, T: Deserialize<'de>, const MIN: usize> Deserialize<'de> for OperandList<T, MIN> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Self::try_new(Vec::<T>::deserialize(deserializer)?).map_err(serde::de::Error::custom)
    }
}
