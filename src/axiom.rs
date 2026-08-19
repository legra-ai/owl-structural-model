//! OWL structural axioms.

use iri_rfc3987::Iri;

use crate::{
    AnnotationSubject, AnnotationValue, AxiomAnnotations, ClassExpression, Entity,
    ObjectPropertyExpression, StructuralModelError,
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
        classes: Vec<ClassExpression>,
        /// Axiom annotations.
        annotations: AxiomAnnotations,
    },
    /// A disjoint-classes axiom.
    DisjointClasses {
        /// Disjoint class expressions.
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
}
