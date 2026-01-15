mod cli;
mod commands;
mod domain;
mod error;
mod platform;
mod ui;

use clap::Parser;
use cli::{Cli, Command};
use platform::{PlatformKind, PlatformRegistry};

fn main() {
    // Parse CLI first to handle --help and --version before platform detection
    let Cli { command } = Cli::parse();

    // Detect available platforms
    let registry = PlatformRegistry::detect();

    if !registry.any_available() {
        eprintln!("Error: No platform tools are available.\n");
        PlatformRegistry::print_unavailable_warnings();
        std::process::exit(1);
    }

    // Validate requested platform is available
    if let Err(msg) = validate_platform_request(&command, &registry) {
        eprintln!("Error: {}", msg);
        std::process::exit(1);
    }

    let result = match command {
        Command::List { ios, android } => commands::list::run(&registry, ios, android),
        Command::Start { ios, android } => commands::start::run(&registry, ios, android),
        Command::Create { ios, android } => commands::create::run(&registry, ios, android),
        Command::Delete { ios, android } => commands::delete::run(&registry, ios, android),
        Command::Erase { ios, android } => commands::erase::run(&registry, ios, android),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// Validates that a requested platform (via CLI flags) is available
fn validate_platform_request(command: &Command, registry: &PlatformRegistry) -> Result<(), String> {
    let (ios, android) = match command {
        Command::List { ios, android }
        | Command::Start { ios, android }
        | Command::Create { ios, android }
        | Command::Delete { ios, android }
        | Command::Erase { ios, android } => (*ios, *android),
    };

    if ios && registry.get(PlatformKind::Ios).is_none() {
        return Err("iOS tools are not available. Make sure Xcode is installed.".to_string());
    }

    if android && registry.get(PlatformKind::Android).is_none() {
        return Err(
            "Android tools are not available. Make sure Android SDK is installed.".to_string(),
        );
    }

    Ok(())
}
