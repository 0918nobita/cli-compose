//! Composable, strict CLI framework with static analysis for Rust

#[cfg(any(feature = "schema", feature = "codegen"))]
mod schema_impl;

#[cfg_attr(
    all(feature = "schema", not(feature = "codegen"), not(feature = "runtime")),
    doc = r##"
```compile_fail
// Cannot use `runtime` mod without `runtime` feature
use cli_compose::runtime::*;
```

```compile_fail
// Cannot use `codegen` mod without `codegen` feature
use cli_compose::codegen::*;
```
"##
)]
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

#[cfg_attr(
    all(feature = "codegen", not(feature = "runtime"), not(feature = "schema")),
    doc = r##"
```compile_fail
// Cannot use `runtime` mod without `runtime` feature
use cli_compose::runtime::*;
```

```compile_fail
// Cannot use `schema` mod without `schema` feature
use cli_compose::schema::*;
```
"##
)]
#[cfg(feature = "codegen")]
pub mod codegen;

#[cfg_attr(
    all(feature = "runtime", not(feature = "codegen"), not(feature = "schema")),
    doc = r##"
```compile_fail
// Cannot use `codegen` mod without `codegen` feature
use cli_compose::codegen::*;
```

```compile_fail
// Cannot use `schema` mod without `schema` feature
use cli_compose::schema::*;
```
"##
)]
#[cfg(feature = "runtime")]
pub mod runtime;
