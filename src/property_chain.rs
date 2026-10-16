//! Ordered property chains and the two forms of object subproperty expressions.

use crate::{ObjectPropertyExpression, StructuralModelError};

/// An ordered chain of at least two object property expressions.
///
/// Order and repeated members are significant. Construction and deserialization
/// enforce the OWL 2 structural arity; ontology-wide regularity and simple-role
/// restrictions require a separate profile check.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(
    try_from = "Vec<ObjectPropertyExpression>",
    into = "Vec<ObjectPropertyExpression>"
)]
pub struct ObjectPropertyChain {
    // bounded: members of one caller-owned structural chain, not a graph relation.
    members: Vec<ObjectPropertyExpression>,
}

impl ObjectPropertyChain {
    /// Create an ordered property chain.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for fewer than two members.
    pub fn try_new(members: Vec<ObjectPropertyExpression>) -> Result<Self, StructuralModelError> {
        if members.len() < 2 {
            return Err(StructuralModelError::InvalidFormat {
                kind: "object property chain",
                value: members.len().to_string(),
                message: "requires at least two object property expressions",
            });
        }
        Ok(Self { members })
    }

    /// Return the members in composition order.
    #[must_use]
    pub fn as_slice(&self) -> &[ObjectPropertyExpression] {
        &self.members
    }
}

impl TryFrom<Vec<ObjectPropertyExpression>> for ObjectPropertyChain {
    type Error = StructuralModelError;

    fn try_from(members: Vec<ObjectPropertyExpression>) -> Result<Self, Self::Error> {
        Self::try_new(members)
    }
}

impl From<ObjectPropertyChain> for Vec<ObjectPropertyExpression> {
    fn from(chain: ObjectPropertyChain) -> Self {
        chain.members
    }
}

/// The subproperty operand of an OWL 2 `SubObjectPropertyOf` axiom.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SubObjectPropertyExpression {
    /// A single named or inverse object property expression.
    Property(ObjectPropertyExpression),
    /// An ordered composition of at least two object property expressions.
    Chain(ObjectPropertyChain),
}
