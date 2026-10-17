//! Checked construction for data axiom families.
use crate::{
    Axiom, AxiomAnnotations, ClassExpression, DataRange, KeyProperties, OperandSet,
    StructuralModelError,
};
use iri_rfc3987::Iri;
impl<L, B> Axiom<L, B> {
    /// Construct `SubDataPropertyOf` without axiom annotations.
    #[must_use]
    pub fn sub_data_property_of(sub_property: Iri<String>, super_property: Iri<String>) -> Self {
        Self::SubDataPropertyOf {
            sub_property,
            super_property,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `EquivalentDataProperties` without axiom annotations.
    /// # Errors
    /// Rejects fewer than the structural minimum number of source occurrences.
    pub fn equivalent_data_properties(
        properties: Vec<Iri<String>>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::EquivalentDataProperties {
            properties: OperandSet::try_new(properties)?,
            annotations: AxiomAnnotations::empty(),
        })
    }
    /// Construct `DisjointDataProperties` without axiom annotations.
    /// # Errors
    /// Rejects fewer than the structural minimum number of source occurrences.
    pub fn disjoint_data_properties(
        properties: Vec<Iri<String>>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::DisjointDataProperties {
            properties: OperandSet::try_new(properties)?,
            annotations: AxiomAnnotations::empty(),
        })
    }
    /// Construct `DataPropertyDomain` without axiom annotations.
    #[must_use]
    pub fn data_property_domain(property: Iri<String>, domain: ClassExpression<L, B>) -> Self {
        Self::DataPropertyDomain {
            property,
            domain,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `DataPropertyRange` without axiom annotations.
    #[must_use]
    pub fn data_property_range(property: Iri<String>, range: DataRange<L>) -> Self {
        Self::DataPropertyRange {
            property,
            range,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `FunctionalDataProperty` without axiom annotations.
    #[must_use]
    pub fn functional_data_property(property: Iri<String>) -> Self {
        Self::FunctionalDataProperty {
            property,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `DatatypeDefinition` without axiom annotations.
    #[must_use]
    pub fn datatype_definition(datatype: Iri<String>, range: DataRange<L>) -> Self {
        Self::DatatypeDefinition {
            datatype,
            range,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `HasKey` without axiom annotations.
    #[must_use]
    pub fn has_key(class: ClassExpression<L, B>, properties: KeyProperties) -> Self {
        Self::HasKey {
            class,
            properties,
            annotations: AxiomAnnotations::empty(),
        }
    }
}
