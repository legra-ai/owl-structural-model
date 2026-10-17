//! Property schema constructors and checked n-ary operands.

use iri_rfc3987::Iri;
use serde::{Deserialize, Deserializer};

use crate::{
    Axiom, AxiomAnnotations, ClassExpression, DataRange, KeyProperties, ObjectPropertyExpression,
    StructuralModelError,
};

impl<L, B> Axiom<L, B> {
    /// Create an object property domain axiom without annotations.
    #[must_use]
    pub fn object_property_domain(
        property: ObjectPropertyExpression,
        domain: ClassExpression,
    ) -> Self {
        Self::ObjectPropertyDomain {
            property,
            domain,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create equivalent object property expressions without annotations.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for fewer than two properties.
    pub fn equivalent_object_properties(
        properties: Vec<ObjectPropertyExpression>,
    ) -> Result<Self, StructuralModelError> {
        Self::require_property_pair(properties.len())?;
        Ok(Self::EquivalentObjectProperties {
            properties,
            annotations: AxiomAnnotations::empty(),
        })
    }

    /// Create pairwise disjoint object property expressions without annotations.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for fewer than two properties.
    pub fn disjoint_object_properties(
        properties: Vec<ObjectPropertyExpression>,
    ) -> Result<Self, StructuralModelError> {
        Self::require_property_pair(properties.len())?;
        Ok(Self::DisjointObjectProperties {
            properties,
            annotations: AxiomAnnotations::empty(),
        })
    }

    /// Create two inverse object property expressions without annotations.
    #[must_use]
    pub fn inverse_object_properties(
        first: ObjectPropertyExpression,
        second: ObjectPropertyExpression,
    ) -> Self {
        Self::InverseObjectProperties {
            first,
            second,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create an object property range without annotations.
    #[must_use]
    pub fn object_property_range(
        property: ObjectPropertyExpression,
        range: ClassExpression,
    ) -> Self {
        Self::ObjectPropertyRange {
            property,
            range,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a data subproperty inclusion without annotations.
    #[must_use]
    pub fn sub_data_property_of(sub_property: Iri<String>, super_property: Iri<String>) -> Self {
        Self::SubDataPropertyOf {
            sub_property,
            super_property,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create equivalent data properties without annotations.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for fewer than two properties.
    pub fn equivalent_data_properties(
        properties: Vec<Iri<String>>,
    ) -> Result<Self, StructuralModelError> {
        Self::require_property_pair(properties.len())?;
        Ok(Self::EquivalentDataProperties {
            properties,
            annotations: AxiomAnnotations::empty(),
        })
    }

    /// Create pairwise disjoint data properties without annotations.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for fewer than two properties.
    pub fn disjoint_data_properties(
        properties: Vec<Iri<String>>,
    ) -> Result<Self, StructuralModelError> {
        Self::require_property_pair(properties.len())?;
        Ok(Self::DisjointDataProperties {
            properties,
            annotations: AxiomAnnotations::empty(),
        })
    }

    /// Create a data property domain without annotations.
    #[must_use]
    pub fn data_property_domain(property: Iri<String>, domain: ClassExpression) -> Self {
        Self::DataPropertyDomain {
            property,
            domain,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a data property range without annotations.
    #[must_use]
    pub fn data_property_range(property: Iri<String>, range: DataRange<L>) -> Self {
        Self::DataPropertyRange {
            property,
            range,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a datatype definition without annotations.
    #[must_use]
    pub fn datatype_definition(datatype: Iri<String>, range: DataRange<L>) -> Self {
        Self::DatatypeDefinition {
            datatype,
            range,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create an object/data property key for instances of a class expression without annotations.
    #[must_use]
    pub fn has_key(class: ClassExpression, properties: KeyProperties) -> Self {
        Self::HasKey {
            class,
            properties,
            annotations: AxiomAnnotations::empty(),
        }
    }

    fn require_property_pair(count: usize) -> Result<(), StructuralModelError> {
        if count < 2 {
            return Err(StructuralModelError::InvalidFormat {
                kind: "property axiom operands",
                value: count.to_string(),
                message: "requires at least two property expressions",
            });
        }
        Ok(())
    }

    pub(crate) fn deserialize_property_pair<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        // bounded: operands of one deserialized structural property axiom.
        let properties = Vec::<T>::deserialize(deserializer)?;
        Self::require_property_pair(properties.len()).map_err(serde::de::Error::custom)?;
        Ok(properties)
    }
}
