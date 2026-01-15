use std::fmt;

/// Custom error type for simls operations
#[derive(Debug)]
pub enum Error {
    /// Command execution failed
    CommandFailed { command: String, message: String },
    /// Failed to parse command output
    Parse { message: String },
    /// User cancelled the operation
    UserCancelled,
    /// No devices available
    NoDevicesAvailable { platform: String },
    /// No platforms available
    NoPlatformsAvailable,
    /// Feature not implemented
    NotImplemented { feature: String },
    /// IO error
    Io(std::io::Error),
    /// JSON parsing error
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CommandFailed { command, message } => {
                write!(f, "Command '{}' failed: {}", command, message)
            }
            Error::Parse { message } => {
                write!(f, "Failed to parse output: {}", message)
            }
            Error::UserCancelled => {
                write!(f, "Operation cancelled by user")
            }
            Error::NoDevicesAvailable { platform } => {
                write!(f, "No {} devices available", platform)
            }
            Error::NoPlatformsAvailable => {
                write!(f, "No platform tools are available")
            }
            Error::NotImplemented { feature } => {
                write!(f, "{} is not yet implemented", feature)
            }
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<dialoguer::Error> for Error {
    fn from(_err: dialoguer::Error) -> Self {
        Error::UserCancelled
    }
}

/// Result type alias for simls operations
pub type Result<T> = std::result::Result<T, Error>;
