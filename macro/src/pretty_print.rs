use std::io::Write;
use std::process;

#[allow(dead_code)]
pub fn pretty_print_rust_code(tokens: proc_macro2::TokenStream) -> anyhow::Result<String> {
    let mut rustfmt = process::Command::new("rustfmt")
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;

    write!(rustfmt.stdin.take().unwrap(), "{}", tokens)?;

    let output = rustfmt.wait_with_output()?;

    let stdout = String::from_utf8(output.stdout)?;

    Ok(stdout)
}
