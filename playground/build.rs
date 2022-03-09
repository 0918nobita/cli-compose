use cli_compose::{AsArgOpt, AsOpt};
use playground_opts::{Output, StdinOpt, StdoutOpt, Verbose};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let dest = std::path::Path::new(&out_dir).join("cli_compose.rs");

    let stdin_flag = format!("{} : {}", StdinOpt::flag(), StdinOpt::description());
    let output_flag = format!("{} : {}", Output::flag(), Output::description());
    let stdout_flag = format!("{} : {}", StdoutOpt::flag(), StdoutOpt::description());
    let verbose_flags = format!("{} : {}", Verbose::flag(), Verbose::description());

    let usage = vec![stdin_flag, output_flag, stdout_flag, verbose_flags].join("\n");

    let contents = quote::quote! {
        pub fn usage() -> String {
            #usage.to_owned()
        }
    }
    .to_string();

    std::fs::write(&dest, contents).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
