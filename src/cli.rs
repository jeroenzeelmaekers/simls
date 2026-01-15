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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_list_command() {
        let cli = Cli::try_parse_from(["simls", "list"]).unwrap();
        match cli.command {
            Command::List { ios, android } => {
                assert!(!ios);
                assert!(!android);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_with_ios_flag() {
        let cli = Cli::try_parse_from(["simls", "list", "--ios"]).unwrap();
        match cli.command {
            Command::List { ios, android } => {
                assert!(ios);
                assert!(!android);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_with_android_flag() {
        let cli = Cli::try_parse_from(["simls", "list", "--android"]).unwrap();
        match cli.command {
            Command::List { ios, android } => {
                assert!(!ios);
                assert!(android);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_with_both_flags() {
        let cli = Cli::try_parse_from(["simls", "list", "--ios", "--android"]).unwrap();
        match cli.command {
            Command::List { ios, android } => {
                assert!(ios);
                assert!(android);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_list_with_short_flags() {
        let cli = Cli::try_parse_from(["simls", "list", "-i", "-a"]).unwrap();
        match cli.command {
            Command::List { ios, android } => {
                assert!(ios);
                assert!(android);
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_cli_start_command() {
        let cli = Cli::try_parse_from(["simls", "start"]).unwrap();
        assert!(matches!(cli.command, Command::Start { .. }));
    }

    #[test]
    fn test_cli_start_with_ios_flag() {
        let cli = Cli::try_parse_from(["simls", "start", "--ios"]).unwrap();
        match cli.command {
            Command::Start { ios, android } => {
                assert!(ios);
                assert!(!android);
            }
            _ => panic!("Expected Start command"),
        }
    }

    #[test]
    fn test_cli_create_command() {
        let cli = Cli::try_parse_from(["simls", "create"]).unwrap();
        assert!(matches!(cli.command, Command::Create { .. }));
    }

    #[test]
    fn test_cli_create_with_android_flag() {
        let cli = Cli::try_parse_from(["simls", "create", "-a"]).unwrap();
        match cli.command {
            Command::Create { ios, android } => {
                assert!(!ios);
                assert!(android);
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_cli_delete_command() {
        let cli = Cli::try_parse_from(["simls", "delete"]).unwrap();
        assert!(matches!(cli.command, Command::Delete { .. }));
    }

    #[test]
    fn test_cli_erase_command() {
        let cli = Cli::try_parse_from(["simls", "erase"]).unwrap();
        assert!(matches!(cli.command, Command::Erase { .. }));
    }

    #[test]
    fn test_cli_invalid_command() {
        let result = Cli::try_parse_from(["simls", "invalid"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_no_command() {
        let result = Cli::try_parse_from(["simls"]);
        assert!(result.is_err());
    }
}
