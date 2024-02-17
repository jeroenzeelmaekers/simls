use clap::Subcommand;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {

    /// list all simulator/emulator
    #[clap(name = "list")]
    List,
}
