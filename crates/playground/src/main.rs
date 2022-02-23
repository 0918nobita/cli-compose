use cli_rs::Arg;

#[derive(Arg)]
/// ソースファイル
struct Input(String);

fn main() {
    println!("Hello, world!");
}
