use clap::Subcommand;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {

    /// start prompt to boot a simulator/emulator
    #[clap(name = "list")]
    List,
}
