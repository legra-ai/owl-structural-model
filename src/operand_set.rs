//! Unordered structural identity without imposing payload traits at construction.

use crate::{OperandList, StructuralModelError};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// An unordered OWL structural association.
/// Equality and hashing ignore order and repeated equal members. Iteration and
/// Serde preserve source occurrences; they do not define a canonical encoding.
/// `MIN` checks functional-syntax occurrence arity, which permits repetitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperandSet<T, const MIN: usize = 0>(OperandList<T, MIN>);
impl<T, const MIN: usize> OperandSet<T, MIN> {
    /// Validate structural occurrence arity without evaluating members.
    /// # Errors
    /// Returns a structural error when fewer than `MIN` occurrences are supplied.
    pub fn try_new(items: Vec<T>) -> Result<Self, StructuralModelError> {
        Ok(Self(OperandList::try_new(items)?))
    }
    /// Return source occurrences, without claiming a canonical order.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}
impl<T> OperandSet<T, 0> {
    /// Create a possibly empty unordered association.
    #[must_use]
    pub fn new(items: Vec<T>) -> Self {
        Self(OperandList::new(items))
    }
}
impl<T, const MIN: usize> Deref for OperandSet<T, MIN> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<'a, T, const MIN: usize> IntoIterator for &'a OperandSet<T, MIN> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}
impl<T: PartialEq, const MIN: usize> PartialEq for OperandSet<T, MIN> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().all(|member| other.contains(member))
            && other.iter().all(|member| self.contains(member))
    }
}
impl<T: Eq, const MIN: usize> Eq for OperandSet<T, MIN> {}
impl<T: Hash, const MIN: usize> Hash for OperandSet<T, MIN> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_unordered(self.as_slice(), state);
    }
}
/// Exactly two unordered source occurrences, including a repeated property.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UnorderedPair<T>([T; 2]);
impl<T> UnorderedPair<T> {
    /// Construct an unordered pair with type-checked arity.
    #[must_use]
    pub fn new(first: T, second: T) -> Self {
        Self([first, second])
    }
    /// Return both source occurrences.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }
}
impl<T: PartialEq> PartialEq for UnorderedPair<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.0[0] == other.0[0] && self.0[1] == other.0[1])
            || (self.0[0] == other.0[1] && self.0[1] == other.0[0])
    }
}
impl<T: Eq> Eq for UnorderedPair<T> {}
impl<T: Hash> Hash for UnorderedPair<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_unordered(&self.0, state);
    }
}
fn hash_unordered<T: Hash, H: Hasher>(members: &[T], state: &mut H) {
    // bounded: one digest per occurrence in this caller-budgeted structural value.
    let mut hashes: Vec<u64> = members
        .iter()
        .map(|member| {
            let mut digest = DefaultHasher::new();
            member.hash(&mut digest);
            digest.finish()
        })
        .collect();
    hashes.sort_unstable();
    hashes.dedup();
    hashes.hash(state);
}
