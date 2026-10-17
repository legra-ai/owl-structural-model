//! All 37 OWL 2 axiom families.

use crate::{
    AnnotationSubject, AnnotationValue, AxiomAnnotations, ClassExpression, DataRange, Entity,
    Individual, KeyProperties, ObjectPropertyExpression, OperandSet, SubObjectPropertyExpression,
    UnorderedPair,
};
use iri_rfc3987::Iri;

/// A structural OWL 2 axiom, independent of a backend or reasoner.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Axiom<L = crate::OwlLiteral, B = crate::BlankNodeId> {
    /// The `Declaration` axiom family.
    Declaration {
        /// The entity operand.
        entity: Entity,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `SubClassOf` axiom family.
    SubClassOf {
        /// The sub class operand.
        sub_class: ClassExpression<L, B>,
        /// The super class operand.
        super_class: ClassExpression<L, B>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `EquivalentClasses` axiom family.
    EquivalentClasses {
        /// The classes operand.
        classes: OperandSet<ClassExpression<L, B>, 2>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `DisjointClasses` axiom family.
    DisjointClasses {
        /// The classes operand.
        classes: OperandSet<ClassExpression<L, B>, 2>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `DisjointUnion` axiom family.
    DisjointUnion {
        /// The class operand.
        class: Iri<String>,
        /// The classes operand.
        classes: OperandSet<ClassExpression<L, B>, 2>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `SubObjectPropertyOf` axiom family.
    SubObjectPropertyOf {
        /// The sub property operand.
        sub_property: SubObjectPropertyExpression,
        /// The super property operand.
        super_property: ObjectPropertyExpression,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `FunctionalObjectProperty` axiom family.
    FunctionalObjectProperty {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `InverseFunctionalObjectProperty` axiom family.
    InverseFunctionalObjectProperty {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `ReflexiveObjectProperty` axiom family.
    ReflexiveObjectProperty {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `IrreflexiveObjectProperty` axiom family.
    IrreflexiveObjectProperty {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `SymmetricObjectProperty` axiom family.
    SymmetricObjectProperty {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `AsymmetricObjectProperty` axiom family.
    AsymmetricObjectProperty {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `TransitiveObjectProperty` axiom family.
    TransitiveObjectProperty {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `EquivalentObjectProperties` axiom family.
    EquivalentObjectProperties {
        /// The properties operand.
        properties: OperandSet<ObjectPropertyExpression, 2>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `DisjointObjectProperties` axiom family.
    DisjointObjectProperties {
        /// The properties operand.
        properties: OperandSet<ObjectPropertyExpression, 2>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `InverseObjectProperties` axiom family.
    InverseObjectProperties {
        /// The properties operand.
        properties: UnorderedPair<ObjectPropertyExpression>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `ObjectPropertyDomain` axiom family.
    ObjectPropertyDomain {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// The domain operand.
        domain: ClassExpression<L, B>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `ObjectPropertyRange` axiom family.
    ObjectPropertyRange {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// The range operand.
        range: ClassExpression<L, B>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `SubDataPropertyOf` axiom family.
    SubDataPropertyOf {
        /// The sub property operand.
        sub_property: Iri<String>,
        /// The super property operand.
        super_property: Iri<String>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `EquivalentDataProperties` axiom family.
    EquivalentDataProperties {
        /// The properties operand.
        properties: OperandSet<Iri<String>, 2>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `DisjointDataProperties` axiom family.
    DisjointDataProperties {
        /// The properties operand.
        properties: OperandSet<Iri<String>, 2>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `DataPropertyDomain` axiom family.
    DataPropertyDomain {
        /// The property operand.
        property: Iri<String>,
        /// The domain operand.
        domain: ClassExpression<L, B>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `DataPropertyRange` axiom family.
    DataPropertyRange {
        /// The property operand.
        property: Iri<String>,
        /// The range operand.
        range: DataRange<L>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `FunctionalDataProperty` axiom family.
    FunctionalDataProperty {
        /// The property operand.
        property: Iri<String>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `DatatypeDefinition` axiom family.
    DatatypeDefinition {
        /// The datatype operand.
        datatype: Iri<String>,
        /// The range operand.
        range: DataRange<L>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `HasKey` axiom family.
    HasKey {
        /// The class operand.
        class: ClassExpression<L, B>,
        /// The properties operand.
        properties: KeyProperties,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `SameIndividual` axiom family.
    SameIndividual {
        /// The individuals operand.
        individuals: OperandSet<Individual<B>, 2>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `DifferentIndividuals` axiom family.
    DifferentIndividuals {
        /// The individuals operand.
        individuals: OperandSet<Individual<B>, 2>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `ClassAssertion` axiom family.
    ClassAssertion {
        /// The class operand.
        class: ClassExpression<L, B>,
        /// The individual operand.
        individual: Individual<B>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `ObjectPropertyAssertion` axiom family.
    ObjectPropertyAssertion {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// The source operand.
        source: Individual<B>,
        /// The target operand.
        target: Individual<B>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `NegativeObjectPropertyAssertion` axiom family.
    NegativeObjectPropertyAssertion {
        /// The property operand.
        property: ObjectPropertyExpression,
        /// The source operand.
        source: Individual<B>,
        /// The target operand.
        target: Individual<B>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `DataPropertyAssertion` axiom family.
    DataPropertyAssertion {
        /// The property operand.
        property: Iri<String>,
        /// The source operand.
        source: Individual<B>,
        /// The value operand.
        value: L,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `NegativeDataPropertyAssertion` axiom family.
    NegativeDataPropertyAssertion {
        /// The property operand.
        property: Iri<String>,
        /// The source operand.
        source: Individual<B>,
        /// The value operand.
        value: L,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `AnnotationAssertion` axiom family.
    AnnotationAssertion {
        /// The property operand.
        property: Iri<String>,
        /// The subject operand.
        subject: AnnotationSubject<B>,
        /// The value operand.
        value: AnnotationValue<L, B>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `SubAnnotationPropertyOf` axiom family.
    SubAnnotationPropertyOf {
        /// The sub property operand.
        sub_property: Iri<String>,
        /// The super property operand.
        super_property: Iri<String>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `AnnotationPropertyDomain` axiom family.
    AnnotationPropertyDomain {
        /// The property operand.
        property: Iri<String>,
        /// The domain operand.
        domain: Iri<String>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
    /// The `AnnotationPropertyRange` axiom family.
    AnnotationPropertyRange {
        /// The property operand.
        property: Iri<String>,
        /// The range operand.
        range: Iri<String>,
        /// Unordered, recursively nested axiom annotations.
        annotations: AxiomAnnotations<L, B>,
    },
}
