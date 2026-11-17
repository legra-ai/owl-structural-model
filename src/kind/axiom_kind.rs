//! The 37 standard OWL 2 axiom families.

use super::catalog::structural_catalog;

structural_catalog! {
    /// A standard OWL 2 axiom family, without backend-specific policy.
    AxiomKind [37] {
        Declaration => "Declaration",
        SubClassOf => "SubClassOf",
        EquivalentClasses => "EquivalentClasses",
        DisjointClasses => "DisjointClasses",
        DisjointUnion => "DisjointUnion",
        SubObjectPropertyOf => "SubObjectPropertyOf",
        FunctionalObjectProperty => "FunctionalObjectProperty",
        InverseFunctionalObjectProperty => "InverseFunctionalObjectProperty",
        ReflexiveObjectProperty => "ReflexiveObjectProperty",
        IrreflexiveObjectProperty => "IrreflexiveObjectProperty",
        SymmetricObjectProperty => "SymmetricObjectProperty",
        AsymmetricObjectProperty => "AsymmetricObjectProperty",
        TransitiveObjectProperty => "TransitiveObjectProperty",
        EquivalentObjectProperties => "EquivalentObjectProperties",
        DisjointObjectProperties => "DisjointObjectProperties",
        InverseObjectProperties => "InverseObjectProperties",
        ObjectPropertyDomain => "ObjectPropertyDomain",
        ObjectPropertyRange => "ObjectPropertyRange",
        SubDataPropertyOf => "SubDataPropertyOf",
        EquivalentDataProperties => "EquivalentDataProperties",
        DisjointDataProperties => "DisjointDataProperties",
        DataPropertyDomain => "DataPropertyDomain",
        DataPropertyRange => "DataPropertyRange",
        FunctionalDataProperty => "FunctionalDataProperty",
        DatatypeDefinition => "DatatypeDefinition",
        HasKey => "HasKey",
        SameIndividual => "SameIndividual",
        DifferentIndividuals => "DifferentIndividuals",
        ClassAssertion => "ClassAssertion",
        ObjectPropertyAssertion => "ObjectPropertyAssertion",
        NegativeObjectPropertyAssertion => "NegativeObjectPropertyAssertion",
        DataPropertyAssertion => "DataPropertyAssertion",
        NegativeDataPropertyAssertion => "NegativeDataPropertyAssertion",
        AnnotationAssertion => "AnnotationAssertion",
        SubAnnotationPropertyOf => "SubAnnotationPropertyOf",
        AnnotationPropertyDomain => "AnnotationPropertyDomain",
        AnnotationPropertyRange => "AnnotationPropertyRange",
    }
}
