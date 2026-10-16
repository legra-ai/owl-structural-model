//! Structural key properties, independent of RDF list representation.

use iri_rfc3987::Iri;

use crate::{ObjectPropertyExpression, StructuralModelError};

/// The object and data properties identifying instances in one `HasKey` axiom.
///
/// Either list can be empty; together they must contain at least one property.
/// Object and data properties remain separate even if their IRIs coincide.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "KeyPropertiesInput")]
pub struct KeyProperties {
    // bounded: object properties in one caller-owned structural key axiom.
    object_properties: Vec<ObjectPropertyExpression>,
    // bounded: data properties in one caller-owned structural key axiom.
    data_properties: Vec<Iri<String>>,
}

impl KeyProperties {
    /// Create key properties while preserving the two property categories.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] when both lists are empty.
    pub fn try_new(
        object_properties: Vec<ObjectPropertyExpression>,
        data_properties: Vec<Iri<String>>,
    ) -> Result<Self, StructuralModelError> {
        if object_properties.is_empty() && data_properties.is_empty() {
            return Err(StructuralModelError::InvalidFormat {
                kind: "key properties",
                value: "0".to_owned(),
                message: "requires at least one key property",
            });
        }
        Ok(Self {
            object_properties,
            data_properties,
        })
    }

    /// Return the object property expressions.
    #[must_use]
    pub fn object_properties(&self) -> &[ObjectPropertyExpression] {
        &self.object_properties
    }

    /// Return the named data properties.
    #[must_use]
    pub fn data_properties(&self) -> &[Iri<String>] {
        &self.data_properties
    }
}

#[derive(serde::Deserialize)]
struct KeyPropertiesInput {
    // bounded: object properties of one deserialized structural key axiom.
    object_properties: Vec<ObjectPropertyExpression>,
    // bounded: data properties of one deserialized structural key axiom.
    data_properties: Vec<Iri<String>>,
}

impl TryFrom<KeyPropertiesInput> for KeyProperties {
    type Error = StructuralModelError;

    fn try_from(input: KeyPropertiesInput) -> Result<Self, Self::Error> {
        Self::try_new(input.object_properties, input.data_properties)
    }
}
