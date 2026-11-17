//! Bounded-depth proptest generators for structural OWL 2 values.

mod axiom;
mod axiom_schema;
mod expression;
mod leaf;

pub use axiom::axiom;
pub use expression::{class_expression, data_range};
pub use leaf::{individual, object_property};
