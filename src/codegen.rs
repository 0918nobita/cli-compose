use crate::schema_impl::{AsCliMeta, CliBuilder, CliBuilderResult};

/// Creates Rust source code generator.
/// The generated code defines a new struct representing the result of parsing
/// and implements [`cli_compose::runtime::AsCli`] trait for the specified struct.
pub fn define_cli<Cli: AsCliMeta>(base_path: &str) -> CliBuilderResult<CliBuilder> {
    CliBuilder::new::<Cli>(base_path)
}
