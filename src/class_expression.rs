//! Complete OWL 2 class expressions with shared caller-owned literal/individual values.

use crate::{Cardinality, ClassExpressionKind, DataRange, Individual, OperandList, OperandSet};
use iri_rfc3987::Iri;

/// A named object property or inverse of a named object property.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ObjectPropertyExpression {
    /// A named property.
    Named(Iri<String>),
    /// The inverse of a named property, as required by the structural grammar.
    InverseOf(Iri<String>),
}
impl ObjectPropertyExpression {
    /// Create a named object property expression.
    #[must_use]
    pub fn named(iri: Iri<String>) -> Self {
        Self::Named(iri)
    }
    /// Create the inverse of a named object property.
    #[must_use]
    pub fn inverse_of(iri: Iri<String>) -> Self {
        Self::InverseOf(iri)
    }
}

/// All 18 OWL 2 class-expression forms.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ClassExpression<L = crate::OwlLiteral, B = crate::BlankNodeId> {
    /// A named class, including owl:Thing and owl:Nothing.
    Named(Iri<String>),
    /// Intersection of at least two source occurrences.
    ObjectIntersectionOf(OperandSet<Self, 2>),
    /// Union of at least two source occurrences.
    ObjectUnionOf(OperandSet<Self, 2>),
    /// Complement of one class expression.
    ObjectComplementOf(Box<Self>),
    /// Enumeration of individuals.
    ObjectOneOf(OperandSet<Individual<B>, 1>),
    /// Existential object restriction.
    ObjectSomeValuesFrom {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// The filler operand.
        filler: Box<Self>,
    },
    /// Universal object restriction.
    ObjectAllValuesFrom {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// The filler operand.
        filler: Box<Self>,
    },
    /// Object value restriction.
    ObjectHasValue {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// The individual operand.
        individual: Individual<B>,
    },
    /// Object self restriction.
    ObjectHasSelf {
        /// The property operand.
        property: ObjectPropertyExpression,
    },
    /// Minimum object cardinality restriction.
    ObjectMinCardinality {
        /// The cardinality operand.
        cardinality: Cardinality,
        /// The property operand.
        property: ObjectPropertyExpression,
        /// The filler operand.
        filler: Box<Self>,
    },
    /// Maximum object cardinality restriction.
    ObjectMaxCardinality {
        /// The cardinality operand.
        cardinality: Cardinality,
        /// The property operand.
        property: ObjectPropertyExpression,
        /// The filler operand.
        filler: Box<Self>,
    },
    /// Exact object cardinality restriction.
    ObjectExactCardinality {
        /// The cardinality operand.
        cardinality: Cardinality,
        /// The property operand.
        property: ObjectPropertyExpression,
        /// The filler operand.
        filler: Box<Self>,
    },
    /// Some data-value restriction, with ordered property positions.
    DataSomeValuesFrom {
        /// The properties operand.
        properties: OperandList<Iri<String>, 1>,
        /// The range operand.
        range: DataRange<L>,
    },
    /// All data-value restriction, with ordered property positions.
    DataAllValuesFrom {
        /// The properties operand.
        properties: OperandList<Iri<String>, 1>,
        /// The range operand.
        range: DataRange<L>,
    },
    /// Data value restriction.
    DataHasValue {
        /// The property operand.
        property: Iri<String>,
        /// The value operand.
        value: L,
    },
    /// Min data cardinality restriction.
    DataMinCardinality {
        /// The cardinality operand.
        cardinality: Cardinality,
        /// The property operand.
        property: Iri<String>,
        /// The range operand.
        range: DataRange<L>,
    },
    /// Max data cardinality restriction.
    DataMaxCardinality {
        /// The cardinality operand.
        cardinality: Cardinality,
        /// The property operand.
        property: Iri<String>,
        /// The range operand.
        range: DataRange<L>,
    },
    /// Exact data cardinality restriction.
    DataExactCardinality {
        /// The cardinality operand.
        cardinality: Cardinality,
        /// The property operand.
        property: Iri<String>,
        /// The range operand.
        range: DataRange<L>,
    },
}

impl<L, B> ClassExpression<L, B> {
    /// Return the structural expression kind without evaluating it.
    #[must_use]
    pub fn kind(&self) -> ClassExpressionKind {
        match self {
            Self::Named(..) => ClassExpressionKind::Named,
            Self::ObjectIntersectionOf(..) => ClassExpressionKind::ObjectIntersectionOf,
            Self::ObjectUnionOf(..) => ClassExpressionKind::ObjectUnionOf,
            Self::ObjectComplementOf(..) => ClassExpressionKind::ObjectComplementOf,
            Self::ObjectOneOf(..) => ClassExpressionKind::ObjectOneOf,
            Self::ObjectSomeValuesFrom { .. } => ClassExpressionKind::ObjectSomeValuesFrom,
            Self::ObjectAllValuesFrom { .. } => ClassExpressionKind::ObjectAllValuesFrom,
            Self::ObjectHasValue { .. } => ClassExpressionKind::ObjectHasValue,
            Self::ObjectHasSelf { .. } => ClassExpressionKind::ObjectHasSelf,
            Self::ObjectMinCardinality { .. } => ClassExpressionKind::ObjectMinCardinality,
            Self::ObjectMaxCardinality { .. } => ClassExpressionKind::ObjectMaxCardinality,
            Self::ObjectExactCardinality { .. } => ClassExpressionKind::ObjectExactCardinality,
            Self::DataSomeValuesFrom { .. } => ClassExpressionKind::DataSomeValuesFrom,
            Self::DataAllValuesFrom { .. } => ClassExpressionKind::DataAllValuesFrom,
            Self::DataHasValue { .. } => ClassExpressionKind::DataHasValue,
            Self::DataMinCardinality { .. } => ClassExpressionKind::DataMinCardinality,
            Self::DataMaxCardinality { .. } => ClassExpressionKind::DataMaxCardinality,
            Self::DataExactCardinality { .. } => ClassExpressionKind::DataExactCardinality,
        }
    }
}
