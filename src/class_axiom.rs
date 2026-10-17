//! Checked construction for class axiom families.
use crate::{Axiom, AxiomAnnotations, ClassExpression, Entity, OperandSet, StructuralModelError};
use iri_rfc3987::Iri;
impl<L, B> Axiom<L, B> {
    /// Construct `Declaration` without axiom annotations.
    #[must_use]
    pub fn declaration(entity: Entity) -> Self {
        Self::Declaration {
            entity,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `SubClassOf` without axiom annotations.
    #[must_use]
    pub fn sub_class_of(
        sub_class: ClassExpression<L, B>,
        super_class: ClassExpression<L, B>,
    ) -> Self {
        Self::SubClassOf {
            sub_class,
            super_class,
            annotations: AxiomAnnotations::empty(),
        }
    }
    /// Construct `EquivalentClasses` without axiom annotations.
    /// # Errors
    /// Rejects fewer than the structural minimum number of source occurrences.
    pub fn equivalent_classes(
        classes: Vec<ClassExpression<L, B>>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::EquivalentClasses {
            classes: OperandSet::try_new(classes)?,
            annotations: AxiomAnnotations::empty(),
        })
    }
    /// Construct `DisjointClasses` without axiom annotations.
    /// # Errors
    /// Rejects fewer than the structural minimum number of source occurrences.
    pub fn disjoint_classes(
        classes: Vec<ClassExpression<L, B>>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::DisjointClasses {
            classes: OperandSet::try_new(classes)?,
            annotations: AxiomAnnotations::empty(),
        })
    }
    /// Construct `DisjointUnion` without axiom annotations.
    /// # Errors
    /// Rejects fewer than the structural minimum number of source occurrences.
    pub fn disjoint_union(
        class: Iri<String>,
        classes: Vec<ClassExpression<L, B>>,
    ) -> Result<Self, StructuralModelError> {
        Ok(Self::DisjointUnion {
            class,
            classes: OperandSet::try_new(classes)?,
            annotations: AxiomAnnotations::empty(),
        })
    }
}
