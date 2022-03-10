use std::fs;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut dest = std::path::PathBuf::from(&out_dir).join("cli_compose");

    fs::create_dir_all(&dest).unwrap();

    dest.push("cli.rs");

    let contents = quote::quote! {
        #[allow(dead_code)]
        struct Cli {
            input: String,
            output: Option<String>,
            stdin: Option<playground_opts::StdinOpt>,
            stdout: Option<playground_opts::StdoutOpt>,
            verbose: Option<playground_opts::Verbose>,
        }

        #[allow(dead_code)]
        impl Cli {
            pub fn parse(args: impl Iterator<Item = String>) {
                let tokens = cli_compose::runtime::parse_into_tokens(args).collect::<Vec<_>>();
                println!("{:?}", tokens);
            }
        }
    }
    .to_string();

    fs::write(&dest, contents).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
