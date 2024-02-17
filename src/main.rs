mod utils;
mod structs;
mod commands;
mod cli;

use clap::Parser;

use cli::{Cli, Command};
use commands::list;

fn main()  {
    let devices = utils::read_ios_simulators().unwrap();

    let Cli { command } = Cli::parse();

    match command {
        Command::List => list::run(devices),
    }
}
