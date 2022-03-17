use example_opts::*;

fn main() {
    cli_compose::codegen::define_cli::<Cli>("example_opts")
        .unwrap()
        .member::<Input>()
        .member::<StdinOpt>()
        .member::<Output>()
        .member::<StdoutOpt>()
        .member::<InputFormat>()
        .member::<Verbose>()
        .build("CliResult")
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
