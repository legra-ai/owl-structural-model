//! Shared structural kind catalogs for visitors, diagnostics and dispatch.

mod axiom_kind;
mod catalog;
mod class_expression_kind;
mod data_range_kind;

pub use axiom_kind::AxiomKind;
pub use class_expression_kind::ClassExpressionKind;
pub use data_range_kind::DataRangeKind;
