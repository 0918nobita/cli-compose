use playground_opts::*;

fn main() {
    cli_compose::codegen::define_cli! {
        Cli -> CliResult {
            playground_opts::Input,
            playground_opts::StdinOpt,
            playground_opts::Output,
            playground_opts::StdoutOpt,
            playground_opts::InputFormat,
            playground_opts::Verbose,
        }
    };

    cli_compose::codegen::define_cli2::<Cli>()
        .pos_arg::<Input>()
        .opt::<StdinOpt>()
        .arg_opt::<Output>()
        .opt::<StdoutOpt>()
        .arg_opt::<InputFormat>()
        .opt::<Verbose>()
        .build("CliResult");

    println!("cargo:rerun-if-changed=build.rs");
}
