mod cli;
mod commands;
mod structs;
mod utils;

use clap::Parser;

use cli::{Cli, Command};
use commands::list;

fn main() {
    let ios_devices = utils::read_ios_simulators().unwrap();
    let android_devices = utils::read_android_emulators().unwrap();

    let Cli { command } = Cli::parse();

    match command {
        Command::List { ios, android } => list::run(ios_devices, android_devices, ios, android),
    }
}
