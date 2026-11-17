//! Typed OWL 2 reserved vocabulary: built-in entities, reserved annotation
//! properties, constraining facets and the OWL 2 datatype map.

mod annotation_property;
mod built_in_entity;
mod catalog;
mod datatype;
mod facet;
mod term;

pub use annotation_property::ReservedAnnotationProperty;
pub use built_in_entity::BuiltInEntity;
pub use datatype::OwlDatatype;
pub use facet::ConstrainingFacet;
pub use term::VocabularyTerm;
