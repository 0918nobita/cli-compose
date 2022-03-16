//! Next-generation, type-safe CLI parser for Rust

#[cfg(feature = "codegen")]
pub mod codegen;

#[cfg(feature = "runtime")]
pub mod runtime;

#[cfg(any(feature = "schema", feature = "codegen"))]
mod schema_impl;

#[cfg(feature = "schema")]
pub mod schema {
    pub use crate::schema_impl::*;
}
