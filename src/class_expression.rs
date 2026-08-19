//! OWL class and object-property expressions.

use iri_rfc3987::Iri;

use crate::StructuralModelError;

const KIND_CLASS_EXPRESSION_OPERANDS: &str = "class expression operands";

/// An OWL object property expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ObjectPropertyExpression {
    /// A named object property.
    Named(Iri<String>),
    /// The inverse of another object property expression.
    InverseOf(Box<ObjectPropertyExpression>),
}

impl ObjectPropertyExpression {
    /// Create a named object property expression.
    #[must_use]
    pub fn named(iri: Iri<String>) -> Self {
        Self::Named(iri)
    }

    /// Create an inverse object property expression.
    #[must_use]
    pub fn inverse_of(property: Self) -> Self {
        Self::InverseOf(Box::new(property))
    }
}

/// An OWL class expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ClassExpression {
    /// A named class.
    Named(Iri<String>),
    /// `owl:Thing`.
    Thing,
    /// `owl:Nothing`.
    Nothing,
    /// An intersection of class expressions.
    ObjectIntersectionOf(Vec<Self>),
    /// A union of class expressions.
    ObjectUnionOf(Vec<Self>),
    /// A complement of a class expression.
    ObjectComplementOf(Box<Self>),
    /// An existential object-property restriction.
    ObjectSomeValuesFrom {
        /// Restricted property.
        property: ObjectPropertyExpression,
        /// Required filler.
        filler: Box<Self>,
    },
    /// A universal object-property restriction.
    ObjectAllValuesFrom {
        /// Restricted property.
        property: ObjectPropertyExpression,
        /// Required filler.
        filler: Box<Self>,
    },
}

impl ClassExpression {
    /// Create a named class expression.
    #[must_use]
    pub fn named(iri: Iri<String>) -> Self {
        Self::Named(iri)
    }

    /// Create an intersection expression.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for zero operands.
    pub fn object_intersection_of(operands: Vec<Self>) -> Result<Self, StructuralModelError> {
        Self::require_operands(&operands)?;
        Ok(Self::ObjectIntersectionOf(operands))
    }

    /// Create a union expression.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for zero operands.
    pub fn object_union_of(operands: Vec<Self>) -> Result<Self, StructuralModelError> {
        Self::require_operands(&operands)?;
        Ok(Self::ObjectUnionOf(operands))
    }

    /// Create a complement expression.
    #[must_use]
    pub fn object_complement_of(operand: Self) -> Self {
        Self::ObjectComplementOf(Box::new(operand))
    }

    /// Create an existential object-property restriction.
    #[must_use]
    pub fn object_some_values_from(property: ObjectPropertyExpression, filler: Self) -> Self {
        Self::ObjectSomeValuesFrom {
            property,
            filler: Box::new(filler),
        }
    }

    /// Create a universal object-property restriction.
    #[must_use]
    pub fn object_all_values_from(property: ObjectPropertyExpression, filler: Self) -> Self {
        Self::ObjectAllValuesFrom {
            property,
            filler: Box::new(filler),
        }
    }

    fn require_operands(operands: &[Self]) -> Result<(), StructuralModelError> {
        if operands.is_empty() {
            return Err(StructuralModelError::InvalidFormat {
                kind: KIND_CLASS_EXPRESSION_OPERANDS,
                value: "0".to_owned(),
                message: "requires at least one operand",
            });
        }
        Ok(())
    }
}
