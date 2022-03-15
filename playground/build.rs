use playground_opts::*;

fn main() {
    cli_compose::codegen::define_cli::<Cli>("playground_opts")
        .unwrap()
        .member::<Input, _>()
        .member::<StdinOpt, _>()
        .member::<Output, _>()
        .member::<StdoutOpt, _>()
        .member::<InputFormat, _>()
        .member::<Verbose, _>()
        .build("CliResult")
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
