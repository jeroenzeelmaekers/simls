mod android;
mod ios;

pub use android::AndroidPlatform;
pub use ios::IosPlatform;

use crate::domain::{Device, DeviceType, Runtime};
use crate::error::Result;
use std::process::Command;

/// Represents the available platform types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformKind {
    Ios,
    Android,
}

impl PlatformKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlatformKind::Ios => "iOS",
            PlatformKind::Android => "Android",
        }
    }
}

impl std::fmt::Display for PlatformKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Platform trait defining operations for simulator/emulator management.
///
/// This trait follows the Interface Segregation Principle (ISP) by providing
/// a focused set of operations. It enables the Open-Closed Principle (OCP)
/// by allowing new platforms to be added without modifying existing code.
pub trait Platform: Send + Sync {
    /// Returns the platform kind
    fn kind(&self) -> PlatformKind;

    /// Returns a human-readable name for the platform
    fn name(&self) -> &'static str {
        self.kind().as_str()
    }

    /// Lists all available devices for this platform
    fn list_devices(&self) -> Result<Vec<Device>>;

    /// Starts a device by its identifier
    fn start_device(&self, device_id: &str) -> Result<()>;

    /// Creates a new device with the given configuration
    fn create_device(&self, name: &str, device_type: &str, runtime: &str) -> Result<()>;

    /// Deletes a device by its identifier
    fn delete_device(&self, device_id: &str) -> Result<()>;

    /// Erases all content and settings from a device
    fn erase_device(&self, device_id: &str) -> Result<()>;

    /// Lists available runtimes/system images
    fn list_runtimes(&self) -> Result<Vec<Runtime>>;

    /// Lists available device types/profiles
    fn list_device_types(&self) -> Result<Vec<DeviceType>>;

    /// Lists runtimes available for download (not yet installed)
    fn list_available_runtimes(&self) -> Result<Vec<Runtime>>;

    /// Installs a runtime by its identifier
    fn install_runtime(&self, runtime_id: &str) -> Result<()>;
}

/// Status of platform tool availability
#[derive(Debug)]
pub struct ToolStatus {
    pub available: bool,
    #[allow(dead_code)] // Part of public API for future use
    pub message: Option<String>,
}

impl ToolStatus {
    pub fn available() -> Self {
        Self {
            available: true,
            message: None,
        }
    }

    pub fn unavailable(message: impl Into<String>) -> Self {
        Self {
            available: false,
            message: Some(message.into()),
        }
    }
}

/// Checks if a command exists and is executable
fn command_exists(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Registry of available platforms.
///
/// This struct manages platform availability and provides a unified
/// interface for accessing platform implementations.
pub struct PlatformRegistry {
    platforms: Vec<Box<dyn Platform>>,
}

impl PlatformRegistry {
    /// Creates a new registry, detecting available platforms
    pub fn detect() -> Self {
        let mut platforms: Vec<Box<dyn Platform>> = Vec::new();

        if let Some(ios) = IosPlatform::try_new() {
            platforms.push(Box::new(ios));
        }

        if let Some(android) = AndroidPlatform::try_new() {
            platforms.push(Box::new(android));
        }

        Self { platforms }
    }

    /// Returns true if at least one platform is available
    pub fn any_available(&self) -> bool {
        !self.platforms.is_empty()
    }

    /// Returns a slice of all available platforms
    pub fn platforms(&self) -> &[Box<dyn Platform>] {
        &self.platforms
    }

    /// Gets a platform by kind, if available
    pub fn get(&self, kind: PlatformKind) -> Option<&dyn Platform> {
        self.platforms
            .iter()
            .find(|p| p.kind() == kind)
            .map(|p| p.as_ref())
    }

    /// Print warnings for platforms that couldn't be initialized
    pub fn print_unavailable_warnings() {
        if !cfg!(target_os = "macos") {
            eprintln!("Warning: iOS simulators are only supported on macOS.\n");
        } else if !command_exists("xcrun", &["--version"]) {
            eprintln!(
                "Warning: iOS support unavailable - Xcode Command Line Tools not found.\n\
                 Install with: xcode-select --install\n"
            );
        }

        if !cfg!(target_os = "macos") {
            eprintln!("Warning: Android emulators are currently only supported on macOS.\n");
        } else if Command::new("emulator")
            .args(["-version"])
            .output()
            .is_err()
        {
            eprintln!(
                "Warning: Android support unavailable - Android Emulator not found.\n\
                 Make sure Android SDK is installed and ANDROID_HOME is set.\n"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_kind_as_str() {
        assert_eq!(PlatformKind::Ios.as_str(), "iOS");
        assert_eq!(PlatformKind::Android.as_str(), "Android");
    }

    #[test]
    fn test_platform_kind_display() {
        assert_eq!(format!("{}", PlatformKind::Ios), "iOS");
        assert_eq!(format!("{}", PlatformKind::Android), "Android");
    }

    #[test]
    fn test_platform_kind_equality() {
        assert_eq!(PlatformKind::Ios, PlatformKind::Ios);
        assert_eq!(PlatformKind::Android, PlatformKind::Android);
        assert_ne!(PlatformKind::Ios, PlatformKind::Android);
    }

    #[test]
    fn test_platform_kind_clone() {
        let kind = PlatformKind::Ios;
        let cloned = kind;
        assert_eq!(kind, cloned);
    }

    #[test]
    fn test_tool_status_available() {
        let status = ToolStatus::available();
        assert!(status.available);
        assert!(status.message.is_none());
    }

    #[test]
    fn test_tool_status_unavailable_with_str() {
        let status = ToolStatus::unavailable("Tool not found");
        assert!(!status.available);
        assert_eq!(status.message, Some("Tool not found".to_string()));
    }

    #[test]
    fn test_tool_status_unavailable_with_string() {
        let status = ToolStatus::unavailable(String::from("Custom error message"));
        assert!(!status.available);
        assert_eq!(status.message, Some("Custom error message".to_string()));
    }

    #[test]
    fn test_command_exists_with_valid_command() {
        // 'echo' is available on all platforms
        assert!(command_exists("echo", &["test"]));
    }

    #[test]
    fn test_command_exists_with_invalid_command() {
        assert!(!command_exists("this_command_does_not_exist_12345", &[]));
    }
}
