use crate::schema_impl::{AsCliMeta, CliBuilder, CliBuilderResult};

pub fn define_cli<Cli: AsCliMeta>(base_path: &str) -> CliBuilderResult<CliBuilder> {
    CliBuilder::new::<Cli>(base_path)
}
