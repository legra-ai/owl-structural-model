//! Checked construction for property schema families.
use crate::{
    Axiom, AxiomAnnotations, ClassExpression, ObjectPropertyExpression, OperandSet,
    StructuralModelError, UnorderedPair,
};
impl<L, B> Axiom<L, B> {
    /// Construct `EquivalentObjectProperties` without axiom annotations.
    /// # Errors
    /// Rejects fewer than the structural minimum number of source occurrences.
    pub fn equivalent_object_properties(
        properties: Vec<ObjectPropertyExpression>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::EquivalentObjectProperties {
            properties: OperandSet::try_new(properties)?,
            annotations: AxiomAnnotations::empty(),
        })
    }
    /// Construct `DisjointObjectProperties` without axiom annotations.
    /// # Errors
    /// Rejects fewer than the structural minimum number of source occurrences.
    pub fn disjoint_object_properties(
        properties: Vec<ObjectPropertyExpression>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::DisjointObjectProperties {
            properties: OperandSet::try_new(properties)?,
            annotations: AxiomAnnotations::empty(),
        })
    }
    /// Construct `InverseObjectProperties` without axiom annotations.
    #[must_use]
    pub fn inverse_object_properties(
        first: ObjectPropertyExpression,
        second: ObjectPropertyExpression,
    ) -> Self {
        Self::InverseObjectProperties {
            properties: UnorderedPair::new(first, second),
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `ObjectPropertyDomain` without axiom annotations.
    #[must_use]
    pub fn object_property_domain(
        property: ObjectPropertyExpression,
        domain: ClassExpression<L, B>,
    ) -> Self {
        Self::ObjectPropertyDomain {
            property,
            domain,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `ObjectPropertyRange` without axiom annotations.
    #[must_use]
    pub fn object_property_range(
        property: ObjectPropertyExpression,
        range: ClassExpression<L, B>,
    ) -> Self {
        Self::ObjectPropertyRange {
            property,
            range,
            annotations: AxiomAnnotations::empty(),
        }
    }
}
