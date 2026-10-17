//! Property-axiom construction and uniform axiom annotations.

use iri_rfc3987::Iri;

use crate::{Axiom, AxiomAnnotations, ObjectPropertyExpression, SubObjectPropertyExpression};

impl<L, B> Axiom<L, B> {
    /// Create an object subproperty or property-chain inclusion without annotations.
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

    /// Create a functional object property axiom without annotations.
    #[must_use]
    pub fn functional_object_property(property: ObjectPropertyExpression) -> Self {
        Self::FunctionalObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a inverse-functional object property axiom without annotations.
    #[must_use]
    pub fn inverse_functional_object_property(property: ObjectPropertyExpression) -> Self {
        Self::InverseFunctionalObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a reflexive object property axiom without annotations.
    #[must_use]
    pub fn reflexive_object_property(property: ObjectPropertyExpression) -> Self {
        Self::ReflexiveObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a irreflexive object property axiom without annotations.
    #[must_use]
    pub fn irreflexive_object_property(property: ObjectPropertyExpression) -> Self {
        Self::IrreflexiveObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a symmetric object property axiom without annotations.
    #[must_use]
    pub fn symmetric_object_property(property: ObjectPropertyExpression) -> Self {
        Self::SymmetricObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a asymmetric object property axiom without annotations.
    #[must_use]
    pub fn asymmetric_object_property(property: ObjectPropertyExpression) -> Self {
        Self::AsymmetricObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a transitive object property axiom without annotations.
    #[must_use]
    pub fn transitive_object_property(property: ObjectPropertyExpression) -> Self {
        Self::TransitiveObjectProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a functional data property axiom without annotations.
    #[must_use]
    pub fn functional_data_property(property: Iri<String>) -> Self {
        Self::FunctionalDataProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Return this axiom's annotations.
    #[must_use]
    pub fn annotations(&self) -> &AxiomAnnotations<L, B> {
        match self {
            Self::Declaration { annotations, .. }
            | Self::SubClassOf { annotations, .. }
            | Self::EquivalentClasses { annotations, .. }
            | Self::DisjointClasses { annotations, .. }
            | Self::ObjectPropertyDomain { annotations, .. }
            | Self::AnnotationAssertion { annotations, .. }
            | Self::SubObjectPropertyOf { annotations, .. }
            | Self::FunctionalObjectProperty { annotations, .. }
            | Self::InverseFunctionalObjectProperty { annotations, .. }
            | Self::ReflexiveObjectProperty { annotations, .. }
            | Self::IrreflexiveObjectProperty { annotations, .. }
            | Self::SymmetricObjectProperty { annotations, .. }
            | Self::AsymmetricObjectProperty { annotations, .. }
            | Self::TransitiveObjectProperty { annotations, .. }
            | Self::EquivalentObjectProperties { annotations, .. }
            | Self::DisjointObjectProperties { annotations, .. }
            | Self::InverseObjectProperties { annotations, .. }
            | Self::ObjectPropertyRange { annotations, .. }
            | Self::SubDataPropertyOf { annotations, .. }
            | Self::EquivalentDataProperties { annotations, .. }
            | Self::DisjointDataProperties { annotations, .. }
            | Self::DataPropertyDomain { annotations, .. }
            | Self::DataPropertyRange { annotations, .. }
            | Self::DatatypeDefinition { annotations, .. }
            | Self::HasKey { annotations, .. }
            | Self::FunctionalDataProperty { annotations, .. } => annotations,
        }
    }

    /// Replace this axiom's annotations, preserving its structural operands.
    #[must_use]
    pub fn with_annotations(mut self, new_annotations: AxiomAnnotations<L, B>) -> Self {
        match &mut self {
            Self::Declaration { annotations, .. }
            | Self::SubClassOf { annotations, .. }
            | Self::EquivalentClasses { annotations, .. }
            | Self::DisjointClasses { annotations, .. }
            | Self::ObjectPropertyDomain { annotations, .. }
            | Self::AnnotationAssertion { annotations, .. }
            | Self::SubObjectPropertyOf { annotations, .. }
            | Self::FunctionalObjectProperty { annotations, .. }
            | Self::InverseFunctionalObjectProperty { annotations, .. }
            | Self::ReflexiveObjectProperty { annotations, .. }
            | Self::IrreflexiveObjectProperty { annotations, .. }
            | Self::SymmetricObjectProperty { annotations, .. }
            | Self::AsymmetricObjectProperty { annotations, .. }
            | Self::TransitiveObjectProperty { annotations, .. }
            | Self::EquivalentObjectProperties { annotations, .. }
            | Self::DisjointObjectProperties { annotations, .. }
            | Self::InverseObjectProperties { annotations, .. }
            | Self::ObjectPropertyRange { annotations, .. }
            | Self::SubDataPropertyOf { annotations, .. }
            | Self::EquivalentDataProperties { annotations, .. }
            | Self::DisjointDataProperties { annotations, .. }
            | Self::DataPropertyDomain { annotations, .. }
            | Self::DataPropertyRange { annotations, .. }
            | Self::DatatypeDefinition { annotations, .. }
            | Self::HasKey { annotations, .. }
            | Self::FunctionalDataProperty { annotations, .. } => *annotations = new_annotations,
        }
        self
    }
}
