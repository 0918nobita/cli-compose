//! Next-generation, type-safe CLI parser for Rust

#[cfg(any(feature = "schema", feature = "codegen"))]
mod schema_impl;

#[cfg(feature = "schema")]
pub mod schema {
    pub use crate::schema_impl::*;

    pub use cli_compose_macro::{
        ArgOpt, Cli, FromKebabStr, MultiSelect, Opt, PosArg, SingleSelect,
    };

    pub mod forwarded {
        pub use quote;
        pub use syn;
    }
}

#[cfg(feature = "codegen")]
pub mod codegen;

#[cfg(feature = "runtime")]
pub mod runtime;
