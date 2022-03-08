use cli_compose::Opt;

#[derive(Debug, Opt)]
#[opt(short = 'V')]
pub struct Verbose;
