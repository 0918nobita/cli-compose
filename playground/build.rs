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

    println!("cargo:rerun-if-changed=build.rs");
}
