//! The 18 standard OWL 2 class-expression forms.

use super::catalog::structural_catalog;

structural_catalog! {
    /// A standard OWL 2 class-expression form, without backend-specific policy.
    ClassExpressionKind [18] {
        Named => "Class",
        ObjectIntersectionOf => "ObjectIntersectionOf",
        ObjectUnionOf => "ObjectUnionOf",
        ObjectComplementOf => "ObjectComplementOf",
        ObjectOneOf => "ObjectOneOf",
        ObjectSomeValuesFrom => "ObjectSomeValuesFrom",
        ObjectAllValuesFrom => "ObjectAllValuesFrom",
        ObjectHasValue => "ObjectHasValue",
        ObjectHasSelf => "ObjectHasSelf",
        ObjectMinCardinality => "ObjectMinCardinality",
        ObjectMaxCardinality => "ObjectMaxCardinality",
        ObjectExactCardinality => "ObjectExactCardinality",
        DataSomeValuesFrom => "DataSomeValuesFrom",
        DataAllValuesFrom => "DataAllValuesFrom",
        DataHasValue => "DataHasValue",
        DataMinCardinality => "DataMinCardinality",
        DataMaxCardinality => "DataMaxCardinality",
        DataExactCardinality => "DataExactCardinality",
    }
}
