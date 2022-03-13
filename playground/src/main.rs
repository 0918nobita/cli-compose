use cli_compose::runtime::{use_cli, AsCli};

use_cli! { playground_opts::Cli }

fn main() {
    let _res = Cli::parse(std::env::args());
}
