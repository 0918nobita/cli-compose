use cli_compose::runtime::{use_cli, AsCli};

use_cli! { playground_opts::Cli }

fn main() {
    Cli::parse(std::env::args());
}
