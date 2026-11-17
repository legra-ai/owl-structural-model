//! Reserved annotation properties of the OWL 2 structural specification (section 5.5).

use super::catalog::vocabulary_catalog;

vocabulary_catalog! {
    /// A built-in annotation property with predefined semantics.
    ReservedAnnotationProperty [9] {
        /// `rdfs:label`, a human-readable label.
        Label => "http://www.w3.org/2000/01/rdf-schema#label",
        /// `rdfs:comment`, a human-readable comment.
        Comment => "http://www.w3.org/2000/01/rdf-schema#comment",
        /// `rdfs:seeAlso`, an IRI that provides additional information.
        SeeAlso => "http://www.w3.org/2000/01/rdf-schema#seeAlso",
        /// `rdfs:isDefinedBy`, an IRI that provides definitional information.
        IsDefinedBy => "http://www.w3.org/2000/01/rdf-schema#isDefinedBy",
        /// `owl:deprecated`, marking an IRI as deprecated.
        Deprecated => "http://www.w3.org/2002/07/owl#deprecated",
        /// `owl:versionInfo`, describing an IRI's version.
        VersionInfo => "http://www.w3.org/2002/07/owl#versionInfo",
        /// `owl:priorVersion`, naming a prior ontology version (section 3.5).
        PriorVersion => "http://www.w3.org/2002/07/owl#priorVersion",
        /// `owl:backwardCompatibleWith`, naming a compatible prior version (section 3.5).
        BackwardCompatibleWith => "http://www.w3.org/2002/07/owl#backwardCompatibleWith",
        /// `owl:incompatibleWith`, naming an incompatible prior version (section 3.5).
        IncompatibleWith => "http://www.w3.org/2002/07/owl#incompatibleWith",
    }
}
