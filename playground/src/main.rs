cli_compose::runtime::use_cli!(Cli);

fn main() {
    Cli::parse(std::env::args());
}
