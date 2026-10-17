//! Structural OWL 2 data ranges and datatype facets.

use iri_rfc3987::Iri;

use crate::{DataRangeKind, OperandSet, OwlLiteral, StructuralModelError};

/// A datatype facet IRI paired with an exact literal restriction value.
///
/// Whether a facet and value belong to a datatype's facet space is a semantic
/// validation concern, not a structural constructor invariant.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct FacetRestriction<L = OwlLiteral> {
    facet: Iri<String>,
    value: L,
}

impl<L> FacetRestriction<L> {
    /// Create a structural facet restriction without evaluating its literal.
    #[must_use]
    pub fn new(facet: Iri<String>, value: L) -> Self {
        Self { facet, value }
    }

    /// Return the constraining facet IRI.
    #[must_use]
    pub fn facet(&self) -> &Iri<String> {
        &self.facet
    }

    /// Return the exact restriction literal.
    #[must_use]
    pub fn value(&self) -> &L {
        &self.value
    }
}

/// A unary OWL 2 data range, independent of any datatype evaluator or RDF store.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum DataRange<L = OwlLiteral> {
    /// A named datatype.
    Datatype(Iri<String>),
    /// Intersection of at least two data ranges.
    // bounded: operands of one caller-owned structural expression.
    DataIntersectionOf(OperandSet<Self, 2>),
    /// Union of at least two data ranges.
    // bounded: operands of one caller-owned structural expression.
    DataUnionOf(OperandSet<Self, 2>),
    /// Complement of one data range.
    DataComplementOf(Box<Self>),
    /// Enumeration of at least one literal, preserving exact lexical forms.
    // bounded: literals in one caller-owned structural enumeration.
    DataOneOf(OperandSet<L, 1>),
    /// A named datatype restricted by at least one facet/value pair.
    DatatypeRestriction {
        /// The restricted datatype.
        datatype: Iri<String>,
        /// Structural restrictions, interpreted conjunctively.
        // bounded: facets of one caller-owned structural restriction.
        restrictions: OperandSet<FacetRestriction<L>, 1>,
    },
}

impl<L> DataRange<L> {
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
        Ok(Self::DataIntersectionOf(OperandSet::try_new(operands)?))
    }

    /// Create a union of data ranges.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for fewer than two operands.
    pub fn data_union_of(operands: Vec<Self>) -> Result<Self, StructuralModelError> {
        Ok(Self::DataUnionOf(OperandSet::try_new(operands)?))
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
    pub fn data_one_of(literals: Vec<L>) -> Result<Self, StructuralModelError> {
        Ok(Self::DataOneOf(OperandSet::try_new(literals)?))
    }

    /// Create a restricted datatype without evaluating its facets or values.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] for zero facet restrictions.
    pub fn datatype_restriction(
        datatype: Iri<String>,
        restrictions: Vec<FacetRestriction<L>>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::DatatypeRestriction {
            datatype,
            restrictions: OperandSet::try_new(restrictions)?,
        })
    }
}

impl<L> DataRange<L> {
    /// Return the standard data-range family without evaluating datatype values.
    #[must_use]
    pub fn kind(&self) -> DataRangeKind {
        match self {
            Self::Datatype(_) => DataRangeKind::Datatype,
            Self::DataIntersectionOf(_) => DataRangeKind::DataIntersectionOf,
            Self::DataUnionOf(_) => DataRangeKind::DataUnionOf,
            Self::DataComplementOf(_) => DataRangeKind::DataComplementOf,
            Self::DataOneOf(_) => DataRangeKind::DataOneOf,
            Self::DatatypeRestriction { .. } => DataRangeKind::DatatypeRestriction,
        }
    }
}
