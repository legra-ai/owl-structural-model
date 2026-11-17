//! Normative constraining facets of the OWL 2 datatype map (section 4).

use super::catalog::vocabulary_catalog;

vocabulary_catalog! {
    /// A constraining facet that OWL 2 implementations must support for some datatype.
    ConstrainingFacet [9] {
        /// `xsd:minInclusive`, for numeric and date-time datatypes.
        MinInclusive => "http://www.w3.org/2001/XMLSchema#minInclusive",
        /// `xsd:maxInclusive`, for numeric and date-time datatypes.
        MaxInclusive => "http://www.w3.org/2001/XMLSchema#maxInclusive",
        /// `xsd:minExclusive`, for numeric and date-time datatypes.
        MinExclusive => "http://www.w3.org/2001/XMLSchema#minExclusive",
        /// `xsd:maxExclusive`, for numeric and date-time datatypes.
        MaxExclusive => "http://www.w3.org/2001/XMLSchema#maxExclusive",
        /// `xsd:length`, for string, binary and IRI datatypes.
        Length => "http://www.w3.org/2001/XMLSchema#length",
        /// `xsd:minLength`, for string, binary and IRI datatypes.
        MinLength => "http://www.w3.org/2001/XMLSchema#minLength",
        /// `xsd:maxLength`, for string, binary and IRI datatypes.
        MaxLength => "http://www.w3.org/2001/XMLSchema#maxLength",
        /// `xsd:pattern`, for string, binary and IRI datatypes.
        Pattern => "http://www.w3.org/2001/XMLSchema#pattern",
        /// `rdf:langRange`, a basic language range for `rdf:PlainLiteral`.
        LangRange => "http://www.w3.org/1999/02/22-rdf-syntax-ns#langRange",
    }
}
