//! Uniform annotations and kind dispatch for every OWL axiom.
use crate::{Axiom, AxiomAnnotations, AxiomKind};
impl<L, B> Axiom<L, B> {
    /// Return the standard axiom family.
    #[must_use]
    pub fn kind(&self) -> AxiomKind {
        match self {
            Self::Declaration { .. } => AxiomKind::Declaration,
            Self::SubClassOf { .. } => AxiomKind::SubClassOf,
            Self::EquivalentClasses { .. } => AxiomKind::EquivalentClasses,
            Self::DisjointClasses { .. } => AxiomKind::DisjointClasses,
            Self::DisjointUnion { .. } => AxiomKind::DisjointUnion,
            Self::SubObjectPropertyOf { .. } => AxiomKind::SubObjectPropertyOf,
            Self::FunctionalObjectProperty { .. } => AxiomKind::FunctionalObjectProperty,
            Self::InverseFunctionalObjectProperty { .. } => {
                AxiomKind::InverseFunctionalObjectProperty
            }
            Self::ReflexiveObjectProperty { .. } => AxiomKind::ReflexiveObjectProperty,
            Self::IrreflexiveObjectProperty { .. } => AxiomKind::IrreflexiveObjectProperty,
            Self::SymmetricObjectProperty { .. } => AxiomKind::SymmetricObjectProperty,
            Self::AsymmetricObjectProperty { .. } => AxiomKind::AsymmetricObjectProperty,
            Self::TransitiveObjectProperty { .. } => AxiomKind::TransitiveObjectProperty,
            Self::EquivalentObjectProperties { .. } => AxiomKind::EquivalentObjectProperties,
            Self::DisjointObjectProperties { .. } => AxiomKind::DisjointObjectProperties,
            Self::InverseObjectProperties { .. } => AxiomKind::InverseObjectProperties,
            Self::ObjectPropertyDomain { .. } => AxiomKind::ObjectPropertyDomain,
            Self::ObjectPropertyRange { .. } => AxiomKind::ObjectPropertyRange,
            Self::SubDataPropertyOf { .. } => AxiomKind::SubDataPropertyOf,
            Self::EquivalentDataProperties { .. } => AxiomKind::EquivalentDataProperties,
            Self::DisjointDataProperties { .. } => AxiomKind::DisjointDataProperties,
            Self::DataPropertyDomain { .. } => AxiomKind::DataPropertyDomain,
            Self::DataPropertyRange { .. } => AxiomKind::DataPropertyRange,
            Self::FunctionalDataProperty { .. } => AxiomKind::FunctionalDataProperty,
            Self::DatatypeDefinition { .. } => AxiomKind::DatatypeDefinition,
            Self::HasKey { .. } => AxiomKind::HasKey,
            Self::SameIndividual { .. } => AxiomKind::SameIndividual,
            Self::DifferentIndividuals { .. } => AxiomKind::DifferentIndividuals,
            Self::ClassAssertion { .. } => AxiomKind::ClassAssertion,
            Self::ObjectPropertyAssertion { .. } => AxiomKind::ObjectPropertyAssertion,
            Self::NegativeObjectPropertyAssertion { .. } => {
                AxiomKind::NegativeObjectPropertyAssertion
            }
            Self::DataPropertyAssertion { .. } => AxiomKind::DataPropertyAssertion,
            Self::NegativeDataPropertyAssertion { .. } => AxiomKind::NegativeDataPropertyAssertion,
            Self::AnnotationAssertion { .. } => AxiomKind::AnnotationAssertion,
            Self::SubAnnotationPropertyOf { .. } => AxiomKind::SubAnnotationPropertyOf,
            Self::AnnotationPropertyDomain { .. } => AxiomKind::AnnotationPropertyDomain,
            Self::AnnotationPropertyRange { .. } => AxiomKind::AnnotationPropertyRange,
        }
    }
    /// Return axiom annotations.
    #[must_use]
    pub fn annotations(&self) -> &AxiomAnnotations<L, B> {
        match self {
            Self::Declaration { annotations, .. }
            | Self::SubClassOf { annotations, .. }
            | Self::EquivalentClasses { annotations, .. }
            | Self::DisjointClasses { annotations, .. }
            | Self::DisjointUnion { annotations, .. }
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
            | Self::ObjectPropertyDomain { annotations, .. }
            | Self::ObjectPropertyRange { annotations, .. }
            | Self::SubDataPropertyOf { annotations, .. }
            | Self::EquivalentDataProperties { annotations, .. }
            | Self::DisjointDataProperties { annotations, .. }
            | Self::DataPropertyDomain { annotations, .. }
            | Self::DataPropertyRange { annotations, .. }
            | Self::FunctionalDataProperty { annotations, .. }
            | Self::DatatypeDefinition { annotations, .. }
            | Self::HasKey { annotations, .. }
            | Self::SameIndividual { annotations, .. }
            | Self::DifferentIndividuals { annotations, .. }
            | Self::ClassAssertion { annotations, .. }
            | Self::ObjectPropertyAssertion { annotations, .. }
            | Self::NegativeObjectPropertyAssertion { annotations, .. }
            | Self::DataPropertyAssertion { annotations, .. }
            | Self::NegativeDataPropertyAssertion { annotations, .. }
            | Self::AnnotationAssertion { annotations, .. }
            | Self::SubAnnotationPropertyOf { annotations, .. }
            | Self::AnnotationPropertyDomain { annotations, .. }
            | Self::AnnotationPropertyRange { annotations, .. } => annotations,
        }
    }
    /// Replace axiom annotations while preserving all logical operands.
    #[must_use]
    pub fn with_annotations(mut self, new_annotations: AxiomAnnotations<L, B>) -> Self {
        match &mut self {
            Self::Declaration { annotations, .. }
            | Self::SubClassOf { annotations, .. }
            | Self::EquivalentClasses { annotations, .. }
            | Self::DisjointClasses { annotations, .. }
            | Self::DisjointUnion { annotations, .. }
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
            | Self::ObjectPropertyDomain { annotations, .. }
            | Self::ObjectPropertyRange { annotations, .. }
            | Self::SubDataPropertyOf { annotations, .. }
            | Self::EquivalentDataProperties { annotations, .. }
            | Self::DisjointDataProperties { annotations, .. }
            | Self::DataPropertyDomain { annotations, .. }
            | Self::DataPropertyRange { annotations, .. }
            | Self::FunctionalDataProperty { annotations, .. }
            | Self::DatatypeDefinition { annotations, .. }
            | Self::HasKey { annotations, .. }
            | Self::SameIndividual { annotations, .. }
            | Self::DifferentIndividuals { annotations, .. }
            | Self::ClassAssertion { annotations, .. }
            | Self::ObjectPropertyAssertion { annotations, .. }
            | Self::NegativeObjectPropertyAssertion { annotations, .. }
            | Self::DataPropertyAssertion { annotations, .. }
            | Self::NegativeDataPropertyAssertion { annotations, .. }
            | Self::AnnotationAssertion { annotations, .. }
            | Self::SubAnnotationPropertyOf { annotations, .. }
            | Self::AnnotationPropertyDomain { annotations, .. }
            | Self::AnnotationPropertyRange { annotations, .. } => *annotations = new_annotations,
        }
        self
    }
}
