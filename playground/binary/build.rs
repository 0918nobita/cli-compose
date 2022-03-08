use cli_compose::AsOpt;
use playground_opts::Verbose;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let dest = std::path::Path::new(&out_dir).join("cli_compose.rs");

    let flags = format!("{}", Verbose::flag());

    let contents = quote::quote! {
        pub fn usage() -> String {
            #flags.to_owned()
        }
    }
    .to_string();

    std::fs::write(&dest, contents).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
