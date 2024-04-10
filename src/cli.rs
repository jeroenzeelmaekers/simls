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

    /// create a simulator/emulator
    #[clap(name = "create")]
    Create {
        #[clap(short, long)]
        ios: bool,
        #[clap(short, long)]
        android: bool,
    },

    /// delete a simulator/emulator
    #[clap(name = "delete")]
    Delete {
        #[clap(short, long)]
        ios: bool,
        #[clap(short, long)]
        android: bool,
    },

    /// erase all settings and content from a simulator/emulator
    #[clap(name = "erase")]
    Erase {
        #[clap(short, long)]
        ios: bool,
        #[clap(short, long)]
        android: bool,
    },
}
