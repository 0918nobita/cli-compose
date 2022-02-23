use cli_rs::Arg;

#[derive(Arg)]
/// ソースファイル
struct Input(String);

fn main() {
    assert_eq!(42, Input::answer());
}
