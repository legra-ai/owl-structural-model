//! Checked construction for property axiom families.
use crate::{Axiom, AxiomAnnotations, ObjectPropertyExpression, SubObjectPropertyExpression};
impl<L, B> Axiom<L, B> {
    /// Construct `SubObjectPropertyOf` without axiom annotations.
    #[must_use]
    pub fn sub_object_property_of(
        sub_property: SubObjectPropertyExpression,
        super_property: ObjectPropertyExpression,
    ) -> Self {
        Self::SubObjectPropertyOf {
            sub_property,
            super_property,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `FunctionalObjectProperty` without axiom annotations.
    #[must_use]
    pub fn functional_object_property(property: ObjectPropertyExpression) -> Self {
        Self::FunctionalObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `InverseFunctionalObjectProperty` without axiom annotations.
    #[must_use]
    pub fn inverse_functional_object_property(property: ObjectPropertyExpression) -> Self {
        Self::InverseFunctionalObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `ReflexiveObjectProperty` without axiom annotations.
    #[must_use]
    pub fn reflexive_object_property(property: ObjectPropertyExpression) -> Self {
        Self::ReflexiveObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `IrreflexiveObjectProperty` without axiom annotations.
    #[must_use]
    pub fn irreflexive_object_property(property: ObjectPropertyExpression) -> Self {
        Self::IrreflexiveObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `SymmetricObjectProperty` without axiom annotations.
    #[must_use]
    pub fn symmetric_object_property(property: ObjectPropertyExpression) -> Self {
        Self::SymmetricObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `AsymmetricObjectProperty` without axiom annotations.
    #[must_use]
    pub fn asymmetric_object_property(property: ObjectPropertyExpression) -> Self {
        Self::AsymmetricObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `TransitiveObjectProperty` without axiom annotations.
    #[must_use]
    pub fn transitive_object_property(property: ObjectPropertyExpression) -> Self {
        Self::TransitiveObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }
}
