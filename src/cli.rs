use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// list all simulator/emulator
    #[clap(name = "list")]
    List {
        #[clap(short, long)]
        ios: bool,
        #[clap(short, long)]
        android: bool,
    },

    /// start a simulator/emulator
    #[clap(name = "start")]
    Start {
        #[clap(short, long)]
        ios: bool,
        #[clap(short, long)]
        android: bool,
    },

    /// stop a running simulator/emulator
    #[clap(name = "stop")]
    Stop,
}
