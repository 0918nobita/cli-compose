include!(concat!(env!("OUT_DIR"), "/cli_compose.rs"));

fn main() {
    println!("{}", usage());
}
