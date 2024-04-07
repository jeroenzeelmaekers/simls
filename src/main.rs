mod cli;
mod commands;
mod structs;
mod utils;

use clap::Parser;
use cli::{Cli, Command};
use commands::{list, start, create};
use utils::{android, ios};

fn main() {
    let ios_devices = ios::read_ios_devices().unwrap();
    let android_devices = android::read_android_emulators().unwrap();

    let Cli { command } = Cli::parse();

    match command {
        Command::List { ios, android } => list::run(ios_devices, android_devices, ios, android),
        Command::Start { ios, android } => start::run(ios_devices, android_devices, ios, android),
        Command::Create { ios, android } => create::run(ios_devices, android_devices, ios, android),
    }
}
