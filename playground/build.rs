fn main() {
    cli_compose::codegen::define_cli!(Cli);

    println!("cargo:rerun-if-changed=build.rs");
}
