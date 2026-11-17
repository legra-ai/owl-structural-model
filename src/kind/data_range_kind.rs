//! The 6 standard OWL 2 data-range forms.

use super::catalog::structural_catalog;

structural_catalog! {
    /// A standard OWL 2 data-range form, without backend-specific policy.
    DataRangeKind [6] {
        Datatype => "Datatype",
        DataIntersectionOf => "DataIntersectionOf",
        DataUnionOf => "DataUnionOf",
        DataComplementOf => "DataComplementOf",
        DataOneOf => "DataOneOf",
        DatatypeRestriction => "DatatypeRestriction",
    }
}
