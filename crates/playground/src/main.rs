use cli_rs::Arg;

#[derive(Arg)]
struct Input(String);

fn main() {
    assert_eq!(42, Input::answer());
}
