//! Shared structural kind catalogs for visitors, diagnostics and dispatch.

/// A standard OWL 2 structural category, without backend-specific policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AxiomKind {
    /// The `Declaration` category.
    Declaration,
    /// The `SubClassOf` category.
    SubClassOf,
    /// The `EquivalentClasses` category.
    EquivalentClasses,
    /// The `DisjointClasses` category.
    DisjointClasses,
    /// The `DisjointUnion` category.
    DisjointUnion,
    /// The `SubObjectPropertyOf` category.
    SubObjectPropertyOf,
    /// The `FunctionalObjectProperty` category.
    FunctionalObjectProperty,
    /// The `InverseFunctionalObjectProperty` category.
    InverseFunctionalObjectProperty,
    /// The `ReflexiveObjectProperty` category.
    ReflexiveObjectProperty,
    /// The `IrreflexiveObjectProperty` category.
    IrreflexiveObjectProperty,
    /// The `SymmetricObjectProperty` category.
    SymmetricObjectProperty,
    /// The `AsymmetricObjectProperty` category.
    AsymmetricObjectProperty,
    /// The `TransitiveObjectProperty` category.
    TransitiveObjectProperty,
    /// The `EquivalentObjectProperties` category.
    EquivalentObjectProperties,
    /// The `DisjointObjectProperties` category.
    DisjointObjectProperties,
    /// The `InverseObjectProperties` category.
    InverseObjectProperties,
    /// The `ObjectPropertyDomain` category.
    ObjectPropertyDomain,
    /// The `ObjectPropertyRange` category.
    ObjectPropertyRange,
    /// The `SubDataPropertyOf` category.
    SubDataPropertyOf,
    /// The `EquivalentDataProperties` category.
    EquivalentDataProperties,
    /// The `DisjointDataProperties` category.
    DisjointDataProperties,
    /// The `DataPropertyDomain` category.
    DataPropertyDomain,
    /// The `DataPropertyRange` category.
    DataPropertyRange,
    /// The `FunctionalDataProperty` category.
    FunctionalDataProperty,
    /// The `DatatypeDefinition` category.
    DatatypeDefinition,
    /// The `HasKey` category.
    HasKey,
    /// The `SameIndividual` category.
    SameIndividual,
    /// The `DifferentIndividuals` category.
    DifferentIndividuals,
    /// The `ClassAssertion` category.
    ClassAssertion,
    /// The `ObjectPropertyAssertion` category.
    ObjectPropertyAssertion,
    /// The `NegativeObjectPropertyAssertion` category.
    NegativeObjectPropertyAssertion,
    /// The `DataPropertyAssertion` category.
    DataPropertyAssertion,
    /// The `NegativeDataPropertyAssertion` category.
    NegativeDataPropertyAssertion,
    /// The `AnnotationAssertion` category.
    AnnotationAssertion,
    /// The `SubAnnotationPropertyOf` category.
    SubAnnotationPropertyOf,
    /// The `AnnotationPropertyDomain` category.
    AnnotationPropertyDomain,
    /// The `AnnotationPropertyRange` category.
    AnnotationPropertyRange,
}

/// A standard OWL 2 structural category, without backend-specific policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ClassExpressionKind {
    /// The `Named` category.
    Named,
    /// The `ObjectIntersectionOf` category.
    ObjectIntersectionOf,
    /// The `ObjectUnionOf` category.
    ObjectUnionOf,
    /// The `ObjectComplementOf` category.
    ObjectComplementOf,
    /// The `ObjectOneOf` category.
    ObjectOneOf,
    /// The `ObjectSomeValuesFrom` category.
    ObjectSomeValuesFrom,
    /// The `ObjectAllValuesFrom` category.
    ObjectAllValuesFrom,
    /// The `ObjectHasValue` category.
    ObjectHasValue,
    /// The `ObjectHasSelf` category.
    ObjectHasSelf,
    /// The `ObjectMinCardinality` category.
    ObjectMinCardinality,
    /// The `ObjectMaxCardinality` category.
    ObjectMaxCardinality,
    /// The `ObjectExactCardinality` category.
    ObjectExactCardinality,
    /// The `DataSomeValuesFrom` category.
    DataSomeValuesFrom,
    /// The `DataAllValuesFrom` category.
    DataAllValuesFrom,
    /// The `DataHasValue` category.
    DataHasValue,
    /// The `DataMinCardinality` category.
    DataMinCardinality,
    /// The `DataMaxCardinality` category.
    DataMaxCardinality,
    /// The `DataExactCardinality` category.
    DataExactCardinality,
}

/// A standard OWL 2 structural category, without backend-specific policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DataRangeKind {
    /// The `Datatype` category.
    Datatype,
    /// The `DataIntersectionOf` category.
    DataIntersectionOf,
    /// The `DataUnionOf` category.
    DataUnionOf,
    /// The `DataComplementOf` category.
    DataComplementOf,
    /// The `DataOneOf` category.
    DataOneOf,
    /// The `DatatypeRestriction` category.
    DatatypeRestriction,
}
