//! Checked construction for assertion families.
use crate::{
    Axiom, AxiomAnnotations, ClassExpression, Individual, ObjectPropertyExpression, OperandSet,
    StructuralModelError,
};
use iri_rfc3987::Iri;
impl<L, B> Axiom<L, B> {
    /// Construct `SameIndividual` without axiom annotations.
    /// # Errors
    /// Rejects fewer than the structural minimum number of source occurrences.
    pub fn same_individual(individuals: Vec<Individual<B>>) -> Result<Self, StructuralModelError> {
        Ok(Self::SameIndividual {
            individuals: OperandSet::try_new(individuals)?,
            annotations: AxiomAnnotations::empty(),
        })
    }
    /// Construct `DifferentIndividuals` without axiom annotations.
    /// # Errors
    /// Rejects fewer than the structural minimum number of source occurrences.
    pub fn different_individuals(
        individuals: Vec<Individual<B>>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::DifferentIndividuals {
            individuals: OperandSet::try_new(individuals)?,
            annotations: AxiomAnnotations::empty(),
        })
    }
    /// Construct `ClassAssertion` without axiom annotations.
    #[must_use]
    pub fn class_assertion(class: ClassExpression<L, B>, individual: Individual<B>) -> Self {
        Self::ClassAssertion {
            class,
            individual,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `ObjectPropertyAssertion` without axiom annotations.
    #[must_use]
    pub fn object_property_assertion(
        property: ObjectPropertyExpression,
        source: Individual<B>,
        target: Individual<B>,
    ) -> Self {
        Self::ObjectPropertyAssertion {
            property,
            source,
            target,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `NegativeObjectPropertyAssertion` without axiom annotations.
    #[must_use]
    pub fn negative_object_property_assertion(
        property: ObjectPropertyExpression,
        source: Individual<B>,
        target: Individual<B>,
    ) -> Self {
        Self::NegativeObjectPropertyAssertion {
            property,
            source,
            target,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `DataPropertyAssertion` without axiom annotations.
    #[must_use]
    pub fn data_property_assertion(property: Iri<String>, source: Individual<B>, value: L) -> Self {
        Self::DataPropertyAssertion {
            property,
            source,
            value,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `NegativeDataPropertyAssertion` without axiom annotations.
    #[must_use]
    pub fn negative_data_property_assertion(
        property: Iri<String>,
        source: Individual<B>,
        value: L,
    ) -> Self {
        Self::NegativeDataPropertyAssertion {
            property,
            source,
            value,
            annotations: AxiomAnnotations::empty(),
        }
    }
}
