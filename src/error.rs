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

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn test_error_display_command_failed() {
        let error = Error::CommandFailed {
            command: "xcrun simctl list".to_string(),
            message: "Command not found".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Command 'xcrun simctl list' failed: Command not found"
        );
    }

    #[test]
    fn test_error_display_parse() {
        let error = Error::Parse {
            message: "Invalid JSON".to_string(),
        };
        assert_eq!(format!("{}", error), "Failed to parse output: Invalid JSON");
    }

    #[test]
    fn test_error_display_user_cancelled() {
        let error = Error::UserCancelled;
        assert_eq!(format!("{}", error), "Operation cancelled by user");
    }

    #[test]
    fn test_error_display_no_devices_available() {
        let error = Error::NoDevicesAvailable {
            platform: "iOS".to_string(),
        };
        assert_eq!(format!("{}", error), "No iOS devices available");
    }

    #[test]
    fn test_error_display_no_platforms_available() {
        let error = Error::NoPlatformsAvailable;
        assert_eq!(format!("{}", error), "No platform tools are available");
    }

    #[test]
    fn test_error_display_not_implemented() {
        let error = Error::NotImplemented {
            feature: "Android emulator erase".to_string(),
        };
        assert_eq!(
            format!("{}", error),
            "Android emulator erase is not yet implemented"
        );
    }

    #[test]
    fn test_error_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error: Error = io_error.into();
        assert!(matches!(error, Error::Io(_)));
        assert!(format!("{}", error).contains("IO error"));
    }

    #[test]
    fn test_error_from_json_error() {
        let json_str = "invalid json {";
        let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
        let error: Error = json_error.into();
        assert!(matches!(error, Error::Json(_)));
        assert!(format!("{}", error).contains("JSON error"));
    }

    #[test]
    fn test_error_source_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = Error::Io(io_error);
        assert!(StdError::source(&error).is_some());
    }

    #[test]
    fn test_error_source_json() {
        let json_str = "invalid json {";
        let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
        let error = Error::Json(json_error);
        assert!(StdError::source(&error).is_some());
    }

    #[test]
    fn test_error_source_none_for_other_variants() {
        assert!(StdError::source(&Error::UserCancelled).is_none());
        assert!(StdError::source(&Error::NoPlatformsAvailable).is_none());
        assert!(StdError::source(&Error::NoDevicesAvailable {
            platform: "test".to_string()
        })
        .is_none());
        assert!(StdError::source(&Error::NotImplemented {
            feature: "test".to_string()
        })
        .is_none());
        assert!(StdError::source(&Error::CommandFailed {
            command: "test".to_string(),
            message: "test".to_string()
        })
        .is_none());
        assert!(StdError::source(&Error::Parse {
            message: "test".to_string()
        })
        .is_none());
    }
}
