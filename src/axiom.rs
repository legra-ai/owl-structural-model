//! OWL structural axioms.

use iri_rfc3987::Iri;

use crate::{
    AnnotationSubject, AnnotationValue, AxiomAnnotations, ClassExpression, DataRange, Entity,
    KeyProperties, ObjectPropertyExpression, StructuralModelError, SubObjectPropertyExpression,
};

const KIND_AXIOM_OPERANDS: &str = "axiom operands";

/// An OWL structural axiom.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Axiom {
    /// An entity declaration.
    Declaration {
        /// Declared OWL entity.
        entity: Entity,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// A subclass axiom.
    SubClassOf {
        /// Subclass expression.
        sub_class: ClassExpression,
        /// Superclass expression.
        super_class: ClassExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An equivalent-classes axiom.
    EquivalentClasses {
        /// Equivalent class expressions.
        #[serde(deserialize_with = "Axiom::deserialize_classes")]
        classes: Vec<ClassExpression>,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// A disjoint-classes axiom.
    DisjointClasses {
        /// Disjoint class expressions.
        #[serde(deserialize_with = "Axiom::deserialize_classes")]
        classes: Vec<ClassExpression>,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An object-property domain axiom.
    ObjectPropertyDomain {
        /// Object property expression.
        property: ObjectPropertyExpression,
        /// Domain class expression.
        domain: ClassExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// Object subproperty inclusion, including ordered property-chain inclusion.
    SubObjectPropertyOf {
        /// A single property expression or an ordered chain.
        sub_property: SubObjectPropertyExpression,
        /// The superproperty expression.
        super_property: ObjectPropertyExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An object property expression is functional.
    FunctionalObjectProperty {
        /// The characterized object property expression.
        property: ObjectPropertyExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An object property expression is inverse-functional.
    InverseFunctionalObjectProperty {
        /// The characterized object property expression.
        property: ObjectPropertyExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An object property expression is reflexive.
    ReflexiveObjectProperty {
        /// The characterized object property expression.
        property: ObjectPropertyExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An object property expression is irreflexive.
    IrreflexiveObjectProperty {
        /// The characterized object property expression.
        property: ObjectPropertyExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An object property expression is symmetric.
    SymmetricObjectProperty {
        /// The characterized object property expression.
        property: ObjectPropertyExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An object property expression is asymmetric.
    AsymmetricObjectProperty {
        /// The characterized object property expression.
        property: ObjectPropertyExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An object property expression is transitive.
    TransitiveObjectProperty {
        /// The characterized object property expression.
        property: ObjectPropertyExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// A data property has at most one distinct data value per individual.
    FunctionalDataProperty {
        /// The named data property.
        property: Iri<String>,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// Equivalent object property expressions.
    EquivalentObjectProperties {
        /// Equivalent object property expressions.
        // bounded: operands of one caller-owned structural property axiom.
        #[serde(deserialize_with = "Axiom::deserialize_property_pair")]
        properties: Vec<ObjectPropertyExpression>,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// Pairwise disjoint object property expressions.
    DisjointObjectProperties {
        /// Pairwise disjoint object property expressions.
        // bounded: operands of one caller-owned structural property axiom.
        #[serde(deserialize_with = "Axiom::deserialize_property_pair")]
        properties: Vec<ObjectPropertyExpression>,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// Two inverse object property expressions.
    InverseObjectProperties {
        /// First object property expression.
        first: ObjectPropertyExpression,
        /// Its inverse object property expression.
        second: ObjectPropertyExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An object property range.
    ObjectPropertyRange {
        /// The restricted object property expression.
        property: ObjectPropertyExpression,
        /// The range class expression.
        range: ClassExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// A data subproperty inclusion.
    SubDataPropertyOf {
        /// The named subproperty.
        sub_property: Iri<String>,
        /// The named superproperty.
        super_property: Iri<String>,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// Equivalent data properties.
    EquivalentDataProperties {
        /// Equivalent named data properties.
        // bounded: operands of one caller-owned structural property axiom.
        #[serde(deserialize_with = "Axiom::deserialize_property_pair")]
        properties: Vec<Iri<String>>,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// Pairwise disjoint data properties.
    DisjointDataProperties {
        /// Pairwise disjoint named data properties.
        // bounded: operands of one caller-owned structural property axiom.
        #[serde(deserialize_with = "Axiom::deserialize_property_pair")]
        properties: Vec<Iri<String>>,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// A data property domain.
    DataPropertyDomain {
        /// The named data property.
        property: Iri<String>,
        /// The domain class expression.
        domain: ClassExpression,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// A data property range.
    DataPropertyRange {
        /// The named data property.
        property: Iri<String>,
        /// The structural data range.
        range: DataRange,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// A datatype definition.
    DatatypeDefinition {
        /// The defined datatype.
        datatype: Iri<String>,
        /// The defining data range.
        range: DataRange,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An object/data property key for instances of a class expression.
    HasKey {
        /// The class expression identified by the key.
        class: ClassExpression,
        /// At least one object or data key property.
        properties: KeyProperties,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// An annotation assertion axiom.
    AnnotationAssertion {
        /// Annotation property IRI.
        property: Iri<String>,
        /// Annotation assertion subject.
        subject: AnnotationSubject,
        /// Annotation assertion value.
        value: AnnotationValue,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
}

impl Axiom {
    /// Create a declaration axiom without axiom annotations.
    #[must_use]
    pub fn declaration(entity: Entity) -> Self {
        Self::Declaration {
            entity,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create a subclass axiom without axiom annotations.
    #[must_use]
    pub fn sub_class_of(sub_class: ClassExpression, super_class: ClassExpression) -> Self {
        Self::SubClassOf {
            sub_class,
            super_class,
            annotations: AxiomAnnotations::empty(),
        }
    }

    /// Create an equivalent-classes axiom without axiom annotations.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] when fewer than two
    /// class expressions are supplied.
    pub fn equivalent_classes(classes: Vec<ClassExpression>) -> Result<Self, StructuralModelError> {
        Self::require_pair(&classes)?;
        Ok(Self::EquivalentClasses {
            classes,
            annotations: AxiomAnnotations::empty(),
        })
    }

    /// Create a disjoint-classes axiom without axiom annotations.
    ///
    /// # Errors
    ///
    /// Returns [`StructuralModelError::InvalidFormat`] when fewer than two
    /// class expressions are supplied.
    pub fn disjoint_classes(classes: Vec<ClassExpression>) -> Result<Self, StructuralModelError> {
        Self::require_pair(&classes)?;
        Ok(Self::DisjointClasses {
            classes,
            annotations: AxiomAnnotations::empty(),
        })
    }

    fn require_pair(classes: &[ClassExpression]) -> Result<(), StructuralModelError> {
        if classes.len() < 2 {
            return Err(StructuralModelError::InvalidFormat {
                kind: KIND_AXIOM_OPERANDS,
                value: classes.len().to_string(),
                message: "requires at least two class expressions",
            });
        }
        Ok(())
    }
    fn deserialize_classes<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<ClassExpression>, D::Error> {
        // bounded: class expressions of one submitted structural axiom.
        let classes = <Vec<ClassExpression> as serde::Deserialize>::deserialize(deserializer)?;
        Self::require_pair(&classes).map_err(serde::de::Error::custom)?;
        Ok(classes)
    }
}
