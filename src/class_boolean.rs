//! Constructors for Boolean class expressions and object restrictions.
use crate::{
    BuiltInEntity, ClassExpression, Individual, ObjectPropertyExpression, OperandSet,
    StructuralModelError, VocabularyTerm,
};
use iri_rfc3987::Iri;

impl<L, B> ClassExpression<L, B> {
    /// Create a named class.
    #[must_use]
    pub fn named(iri: Iri<String>) -> Self {
        Self::Named(iri)
    }
    /// Create owl:Thing as a named class.
    #[must_use]
    pub fn thing() -> Self {
        Self::named(BuiltInEntity::Thing.iri())
    }
    /// Create owl:Nothing as a named class.
    #[must_use]
    pub fn nothing() -> Self {
        Self::named(BuiltInEntity::Nothing.iri())
    }
    /// Return whether this expression is the named universal class.
    #[must_use]
    pub fn is_thing(&self) -> bool {
        matches!(self, Self::Named(iri) if BuiltInEntity::Thing.names(iri))
    }
    /// Return whether this expression is the named empty class.
    #[must_use]
    pub fn is_nothing(&self) -> bool {
        matches!(self, Self::Named(iri) if BuiltInEntity::Nothing.names(iri))
    }
    /// Create an unordered intersection.
    /// # Errors
    /// Rejects fewer than two source occurrences.
    pub fn object_intersection_of(operands: Vec<Self>) -> Result<Self, StructuralModelError> {
        Ok(Self::ObjectIntersectionOf(OperandSet::try_new(operands)?))
    }
    /// Create an unordered union.
    /// # Errors
    /// Rejects fewer than two source occurrences.
    pub fn object_union_of(operands: Vec<Self>) -> Result<Self, StructuralModelError> {
        Ok(Self::ObjectUnionOf(OperandSet::try_new(operands)?))
    }
    /// Create a complement expression.
    #[must_use]
    pub fn object_complement_of(operand: Self) -> Self {
        Self::ObjectComplementOf(Box::new(operand))
    }
    /// Create a nonempty enumeration of individuals.
    /// # Errors
    /// Rejects an empty enumeration.
    pub fn object_one_of(individuals: Vec<Individual<B>>) -> Result<Self, StructuralModelError> {
        Ok(Self::ObjectOneOf(OperandSet::try_new(individuals)?))
    }
    /// Create an object some-values restriction.
    #[must_use]
    pub fn object_some_values_from(property: ObjectPropertyExpression, filler: Self) -> Self {
        Self::ObjectSomeValuesFrom {
            property,
            filler: Box::new(filler),
        }
    }
    /// Create an object all-values restriction.
    #[must_use]
    pub fn object_all_values_from(property: ObjectPropertyExpression, filler: Self) -> Self {
        Self::ObjectAllValuesFrom {
            property,
            filler: Box::new(filler),
        }
    }
    /// Create an object value restriction.
    #[must_use]
    pub fn object_has_value(property: ObjectPropertyExpression, individual: Individual<B>) -> Self {
        Self::ObjectHasValue {
            property,
            individual,
        }
    }
    /// Create a self restriction.
    #[must_use]
    pub fn object_has_self(property: ObjectPropertyExpression) -> Self {
        Self::ObjectHasSelf { property }
    }
}
