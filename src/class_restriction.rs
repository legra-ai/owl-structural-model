//! Cardinality and data-property restriction constructors.
use crate::ObjectPropertyExpression;
use crate::{
    Cardinality, ClassExpression, DataRange, OperandList, RDFS_LITERAL, StructuralModelError,
};
use iri_rfc3987::Iri;
impl<L, B> ClassExpression<L, B> {
    /// Create an object minimum cardinality restriction; no filler means owl:Thing.
    #[must_use]
    pub fn object_min_cardinality(
        cardinality: Cardinality,
        property: ObjectPropertyExpression,
        filler: Option<Self>,
    ) -> Self {
        Self::ObjectMinCardinality {
            cardinality,
            property,
            filler: Box::new(filler.unwrap_or_else(Self::thing)),
        }
    }
    /// Create an object maximum cardinality restriction; no filler means owl:Thing.
    #[must_use]
    pub fn object_max_cardinality(
        cardinality: Cardinality,
        property: ObjectPropertyExpression,
        filler: Option<Self>,
    ) -> Self {
        Self::ObjectMaxCardinality {
            cardinality,
            property,
            filler: Box::new(filler.unwrap_or_else(Self::thing)),
        }
    }
    /// Create an object exact cardinality restriction; no filler means owl:Thing.
    #[must_use]
    pub fn object_exact_cardinality(
        cardinality: Cardinality,
        property: ObjectPropertyExpression,
        filler: Option<Self>,
    ) -> Self {
        Self::ObjectExactCardinality {
            cardinality,
            property,
            filler: Box::new(filler.unwrap_or_else(Self::thing)),
        }
    }
    /// Create a data some-values restriction with ordered property positions.
    /// # Errors
    /// Rejects an empty property list. Datatype arity is checked by semantic validators.
    pub fn data_some_values_from(
        properties: Vec<Iri<String>>,
        range: DataRange<L>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::DataSomeValuesFrom {
            properties: OperandList::try_new(properties)?,
            range,
        })
    }
    /// Create a data all-values restriction with ordered property positions.
    /// # Errors
    /// Rejects an empty property list. Datatype arity is checked by semantic validators.
    pub fn data_all_values_from(
        properties: Vec<Iri<String>>,
        range: DataRange<L>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::DataAllValuesFrom {
            properties: OperandList::try_new(properties)?,
            range,
        })
    }
    /// Create a data-value restriction without evaluating the literal.
    #[must_use]
    pub fn data_has_value(property: Iri<String>, value: L) -> Self {
        Self::DataHasValue { property, value }
    }
    /// Create a data minimum cardinality restriction; no range means rdfs:Literal.
    #[must_use]
    pub fn data_min_cardinality(
        cardinality: Cardinality,
        property: Iri<String>,
        range: Option<DataRange<L>>,
    ) -> Self {
        Self::DataMinCardinality {
            cardinality,
            property,
            range: range.unwrap_or_else(|| DataRange::datatype(Self::builtin(RDFS_LITERAL))),
        }
    }
    /// Create a data maximum cardinality restriction; no range means rdfs:Literal.
    #[must_use]
    pub fn data_max_cardinality(
        cardinality: Cardinality,
        property: Iri<String>,
        range: Option<DataRange<L>>,
    ) -> Self {
        Self::DataMaxCardinality {
            cardinality,
            property,
            range: range.unwrap_or_else(|| DataRange::datatype(Self::builtin(RDFS_LITERAL))),
        }
    }
    /// Create a data exact cardinality restriction; no range means rdfs:Literal.
    #[must_use]
    pub fn data_exact_cardinality(
        cardinality: Cardinality,
        property: Iri<String>,
        range: Option<DataRange<L>>,
    ) -> Self {
        Self::DataExactCardinality {
            cardinality,
            property,
            range: range.unwrap_or_else(|| DataRange::datatype(Self::builtin(RDFS_LITERAL))),
        }
    }
}
