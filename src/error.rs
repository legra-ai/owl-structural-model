//! Errors raised by structural constructors.

use std::fmt::{Display, Formatter};

/// A structural value failed an invariant at construction time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralModelError {
    /// A required value was empty.
    Empty {
        /// The kind of value that was empty.
        kind: &'static str,
    },
    /// A value had the wrong structural shape.
    InvalidFormat {
        /// The kind of value that was invalid.
        kind: &'static str,
        /// The supplied representation.
        value: String,
        /// The required shape.
        message: &'static str,
    },
}

impl Display for StructuralModelError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty { kind } => write!(formatter, "{kind} must not be empty"),
            Self::InvalidFormat {
                kind,
                value,
                message,
            } => write!(formatter, "invalid {kind}: {message} (got: {value})"),
        }
    }
}

impl std::error::Error for StructuralModelError {}
