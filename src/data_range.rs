//! Structural OWL 2 data ranges and datatype facets.

use iri_rfc3987::Iri;

use crate::{OwlLiteral, StructuralModelError};

/// A datatype facet IRI paired with an exact literal restriction value.
///
/// Whether a facet and value belong to a datatype's facet space is a semantic
/// validation concern, not a structural constructor invariant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FacetRestriction {
    facet: Iri<String>,
    value: OwlLiteral,
}

impl FacetRestriction {
    /// Create a structural facet restriction without evaluating its literal.
    #[must_use]
    pub fn new(facet: Iri<String>, value: OwlLiteral) -> Self {
        Self { facet, value }
    }

    /// Return the constraining facet IRI.
    #[must_use]
    pub fn facet(&self) -> &Iri<String> {
        &self.facet
    }

    /// Return the exact restriction literal.
    #[must_use]
    pub fn value(&self) -> &OwlLiteral {
        &self.value
    }
}

/// A unary OWL 2 data range, independent of any datatype evaluator or RDF store.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "DataRangeInput")]
pub enum DataRange {
    /// A named datatype.
    Datatype(Iri<String>),
    /// Intersection of at least two data ranges.
    // bounded: operands of one caller-owned structural expression.
    DataIntersectionOf(Vec<Self>),
    /// Union of at least two data ranges.
    // bounded: operands of one caller-owned structural expression.
    DataUnionOf(Vec<Self>),
    /// Complement of one data range.
    DataComplementOf(Box<Self>),
    /// Enumeration of at least one literal, preserving exact lexical forms.
    // bounded: literals in one caller-owned structural enumeration.
    DataOneOf(Vec<OwlLiteral>),
    /// A named datatype restricted by at least one facet/value pair.
    DatatypeRestriction {
        /// The restricted datatype.
        datatype: Iri<String>,
        /// Structural restrictions, interpreted conjunctively.
        // bounded: facets of one caller-owned structural restriction.
        restrictions: Vec<FacetRestriction>,
    },
}

impl DataRange {
    /// Create a named datatype range.
    #[must_use]
    pub fn datatype(datatype: Iri<String>) -> Self {
        Self::Datatype(datatype)
    }

    /// Create an intersection of data ranges.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for fewer than two operands.
    pub fn data_intersection_of(operands: Vec<Self>) -> Result<Self, StructuralModelError> {
        Self::require_count(operands.len(), 2, "data intersection")?;
        Ok(Self::DataIntersectionOf(operands))
    }

    /// Create a union of data ranges.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for fewer than two operands.
    pub fn data_union_of(operands: Vec<Self>) -> Result<Self, StructuralModelError> {
        Self::require_count(operands.len(), 2, "data union")?;
        Ok(Self::DataUnionOf(operands))
    }

    /// Create the complement of a data range.
    #[must_use]
    pub fn data_complement_of(operand: Self) -> Self {
        Self::DataComplementOf(Box::new(operand))
    }

    /// Create a literal enumeration without normalizing datatype values.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for zero literals.
    pub fn data_one_of(literals: Vec<OwlLiteral>) -> Result<Self, StructuralModelError> {
        Self::require_count(literals.len(), 1, "data enumeration")?;
        Ok(Self::DataOneOf(literals))
    }

    /// Create a restricted datatype without evaluating its facets or values.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for zero facet restrictions.
    pub fn datatype_restriction(
        datatype: Iri<String>,
        restrictions: Vec<FacetRestriction>,
    ) -> Result<Self, StructuralModelError> {
        Self::require_count(restrictions.len(), 1, "datatype restriction")?;
        Ok(Self::DatatypeRestriction {
            datatype,
            restrictions,
        })
    }

    fn require_count(
        count: usize,
        minimum: usize,
        kind: &'static str,
    ) -> Result<(), StructuralModelError> {
        if count < minimum {
            return Err(StructuralModelError::InvalidFormat {
                kind,
                value: count.to_string(),
                message: if minimum == 1 {
                    "requires at least one operand"
                } else {
                    "requires at least two operands"
                },
            });
        }
        Ok(())
    }
}

/// Serde input mirrors the serialized shape; conversion uses the same checked
/// constructors as callers. Recursive operands are checked `DataRange` values.
#[derive(serde::Deserialize)]
enum DataRangeInput {
    Datatype(Iri<String>),
    // bounded: operands of one deserialized structural expression.
    DataIntersectionOf(Vec<DataRange>),
    // bounded: operands of one deserialized structural expression.
    DataUnionOf(Vec<DataRange>),
    DataComplementOf(Box<DataRange>),
    // bounded: literals of one deserialized structural enumeration.
    DataOneOf(Vec<OwlLiteral>),
    DatatypeRestriction {
        datatype: Iri<String>,
        // bounded: facets of one deserialized structural restriction.
        restrictions: Vec<FacetRestriction>,
    },
}

impl TryFrom<DataRangeInput> for DataRange {
    type Error = StructuralModelError;

    fn try_from(input: DataRangeInput) -> Result<Self, Self::Error> {
        match input {
            DataRangeInput::Datatype(datatype) => Ok(Self::datatype(datatype)),
            DataRangeInput::DataIntersectionOf(operands) => Self::data_intersection_of(operands),
            DataRangeInput::DataUnionOf(operands) => Self::data_union_of(operands),
            DataRangeInput::DataComplementOf(operand) => Ok(Self::DataComplementOf(operand)),
            DataRangeInput::DataOneOf(literals) => Self::data_one_of(literals),
            DataRangeInput::DatatypeRestriction {
                datatype,
                restrictions,
            } => Self::datatype_restriction(datatype, restrictions),
        }
    }
}
