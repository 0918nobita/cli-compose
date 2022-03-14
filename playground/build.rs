use playground_opts::Cli;

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
        .pos_arg::<playground_opts::Input>()
        .opt::<playground_opts::StdinOpt>()
        .arg_opt::<playground_opts::Output>()
        .opt::<playground_opts::StdoutOpt>()
        .arg_opt::<playground_opts::InputFormat>()
        .opt::<playground_opts::Verbose>()
        .build("CliResult");

    println!("cargo:rerun-if-changed=build.rs");
}
