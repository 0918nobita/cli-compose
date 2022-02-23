use cli_rs::{Arg, ToArg};

#[derive(Debug, Arg)]
/// ソースファイル
struct Input(String);

fn main() {
    println!("name: {}", Input::name());
    println!("desc: {}", Input::description());
    println!("result: {:?}", Input::parse("hoge"));
}
