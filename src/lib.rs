//! Next-generation, type-safe CLI parser for Rust

#[cfg(feature = "codegen")]
pub mod codegen;

#[cfg(feature = "runtime")]
pub mod runtime;

#[cfg(feature = "schema")]
pub mod schema;
