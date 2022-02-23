use cli_rs::{Arg, ToArg};

#[derive(Arg)]
/// ソースファイル
struct Input(String);

fn main() {
    println!("name: {}", Input::name());
    println!("desc: {}", Input::description());
}
