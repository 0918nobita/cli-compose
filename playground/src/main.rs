include!(concat!(env!("OUT_DIR"), "/cli_compose.rs"));

fn main() {
    Cli::parse(std::env::args());
}
