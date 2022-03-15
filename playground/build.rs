use playground_opts::*;

fn main() {
    cli_compose::codegen::CliBuilder::new::<Cli>("playground_opts")
        .unwrap()
        .pos_arg::<Input>()
        .opt::<StdinOpt>()
        .arg_opt::<Output>()
        .opt::<StdoutOpt>()
        .arg_opt::<InputFormat>()
        .opt::<Verbose>()
        .build("CliResult")
        .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
