fn main() {
    cli_compose::codegen::define_cli!(Cli -> CliResult);

    println!("cargo:rerun-if-changed=build.rs");
}
