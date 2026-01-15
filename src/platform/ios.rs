use crate::domain::{Device, DeviceState, DeviceType, Runtime};
use crate::error::{Error, Result};
use crate::platform::{command_exists, Platform, PlatformKind, ToolStatus};
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

/// iOS Platform implementation using xcrun simctl
pub struct IosPlatform {
    _private: (), // Prevent external construction
}

impl IosPlatform {
    /// Attempts to create an iOS platform instance.
    /// Returns None if iOS tools are not available.
    pub fn try_new() -> Option<Self> {
        let status = Self::check_tools();
        if status.available {
            Some(Self { _private: () })
        } else {
            None
        }
    }

    /// Check if iOS development tools are available
    pub fn check_tools() -> ToolStatus {
        if !cfg!(target_os = "macos") {
            return ToolStatus::unavailable("iOS simulators are only supported on macOS.");
        }

        if !command_exists("xcrun", &["--version"]) {
            return ToolStatus::unavailable(
                "Xcode Command Line Tools not found. Install with: xcode-select --install",
            );
        }

        if !command_exists("xcrun", &["simctl", "help"]) {
            return ToolStatus::unavailable(
                "Xcode Simulator tools not found. \
                 Make sure Xcode is installed and run: \
                 sudo xcode-select -s /Applications/Xcode.app/Contents/Developer",
            );
        }

        ToolStatus::available()
    }

    /// Extracts iOS version from platform identifier string
    fn extract_version(input: &str) -> Option<String> {
        let parts: Vec<&str> = input.split(&['.', '-'][..]).collect();
        for i in 0..parts.len() {
            if parts[i] == "iOS" && i + 2 < parts.len() {
                if let (Ok(major), Ok(minor)) =
                    (parts[i + 1].parse::<u32>(), parts[i + 2].parse::<u32>())
                {
                    return Some(format!("{}.{}", major, minor));
                }
            }
        }
        None
    }

    /// Extracts version number from a line containing iOS version info
    fn extract_version_from_line(line: &str) -> Option<String> {
        // Look for patterns like "iOS 17.0", "iOS 16.4", etc.
        let words: Vec<&str> = line.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            if *word == "iOS" && i + 1 < words.len() {
                let version_candidate =
                    words[i + 1].trim_matches(|c: char| !c.is_numeric() && c != '.');
                if version_candidate.contains('.') {
                    return Some(version_candidate.to_string());
                }
            }
        }
        None
    }

    /// Fallback method to list available runtimes using xcrun simctl
    fn list_available_runtimes_fallback(&self) -> Result<Vec<Runtime>> {
        // Use xcrun simctl runtime to list available runtimes
        let output = Command::new("xcrun")
            .args(["simctl", "runtime", "list", "-j"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Try to parse the JSON output for available runtimes
                if let Ok(available) = serde_json::from_str::<AvailableRuntimes>(&output_str) {
                    let runtimes: Vec<Runtime> = available
                        .runtimes
                        .into_iter()
                        .filter(|r| r.platform.to_lowercase().contains("ios"))
                        .map(|r| Runtime::new(&r.identifier, &r.version))
                        .collect();
                    if !runtimes.is_empty() {
                        return Ok(runtimes);
                    }
                }
            }
        }

        // If all else fails, return common iOS versions as suggestions
        Ok(vec![Runtime::new("iOS", "iOS")])
    }

    /// Install runtime via xcrun simctl runtime add
    fn install_runtime_via_simctl(&self, runtime_id: &str) -> Result<()> {
        let output = Command::new("xcrun")
            .args(["simctl", "runtime", "add", runtime_id])
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: format!("xcrun simctl runtime add {}", runtime_id),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Ok(())
    }
}

impl Platform for IosPlatform {
    fn kind(&self) -> PlatformKind {
        PlatformKind::Ios
    }

    fn list_devices(&self) -> Result<Vec<Device>> {
        let output = Command::new("xcrun")
            .args(["simctl", "list", "--json", "devices"])
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "xcrun simctl list devices".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let output_string = String::from_utf8(output.stdout).map_err(|e| Error::Parse {
            message: format!("Invalid UTF-8 in command output: {}", e),
        })?;

        let raw: RawDevices = serde_json::from_str(&output_string)?;

        let mut devices = Vec::new();
        for (platform, device_list) in raw.devices {
            let version = Self::extract_version(&platform);
            for raw_device in device_list {
                let mut device = Device::new(&raw_device.udid, &raw_device.name)
                    .with_state(DeviceState::from(raw_device.state.as_str()));
                if let Some(ref v) = version {
                    device = device.with_platform_version(v);
                }
                devices.push(device);
            }
        }

        Ok(devices)
    }

    fn start_device(&self, device_id: &str) -> Result<()> {
        let boot_output = Command::new("xcrun")
            .args(["simctl", "boot", device_id])
            .output()?;

        // Ignore "already booted" errors
        if !boot_output.status.success() {
            let stderr = String::from_utf8_lossy(&boot_output.stderr);
            if !stderr.contains("current state: Booted") {
                return Err(Error::CommandFailed {
                    command: "xcrun simctl boot".to_string(),
                    message: stderr.to_string(),
                });
            }
        }

        let open_output = Command::new("open").args(["-a", "Simulator"]).output()?;

        if !open_output.status.success() {
            return Err(Error::CommandFailed {
                command: "open -a Simulator".to_string(),
                message: String::from_utf8_lossy(&open_output.stderr).to_string(),
            });
        }

        // Bring simulator window to front (best effort)
        let _ = Command::new("osascript")
            .args([
                "-e",
                r#"if running of application "Simulator" then
                    tell the application "Simulator" to activate
                end if"#,
            ])
            .output();

        Ok(())
    }

    fn create_device(&self, name: &str, device_type: &str, runtime: &str) -> Result<()> {
        let output = Command::new("xcrun")
            .args(["simctl", "create", name, device_type, runtime])
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "xcrun simctl create".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Ok(())
    }

    fn delete_device(&self, device_id: &str) -> Result<()> {
        let output = Command::new("xcrun")
            .args(["simctl", "delete", device_id])
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "xcrun simctl delete".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Ok(())
    }

    fn erase_device(&self, device_id: &str) -> Result<()> {
        let output = Command::new("xcrun")
            .args(["simctl", "erase", device_id])
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "xcrun simctl erase".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Ok(())
    }

    fn list_runtimes(&self) -> Result<Vec<Runtime>> {
        let output = Command::new("xcrun")
            .args(["simctl", "list", "runtimes", "--json"])
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "xcrun simctl list runtimes".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let output_string = String::from_utf8(output.stdout).map_err(|e| Error::Parse {
            message: format!("Invalid UTF-8 in command output: {}", e),
        })?;

        let raw: RawRuntimes = serde_json::from_str(&output_string)?;

        let runtimes = raw
            .runtimes
            .into_iter()
            .map(|r| {
                let device_types = r
                    .supported_device_types
                    .into_iter()
                    .map(|dt| DeviceType::new(&dt.identifier, &dt.name))
                    .collect();
                Runtime::new(&r.identifier, &r.version).with_device_types(device_types)
            })
            .collect();

        Ok(runtimes)
    }

    fn list_device_types(&self) -> Result<Vec<DeviceType>> {
        // For iOS, device types are retrieved per-runtime via list_runtimes
        // This returns all unique device types across all runtimes
        let runtimes = self.list_runtimes()?;
        let mut device_types = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for runtime in runtimes {
            for dt in runtime.supported_device_types {
                if seen.insert(dt.identifier.clone()) {
                    device_types.push(dt);
                }
            }
        }

        Ok(device_types)
    }

    fn list_available_runtimes(&self) -> Result<Vec<Runtime>> {
        // Use xcodebuild to list downloadable platforms
        let output = Command::new("xcodebuild")
            .args(["-downloadAllPlatforms", "-dry-run"])
            .output();

        // If the dry-run command fails or isn't supported, try parsing xcrun output
        // and use Apple's platform list (this is a fallback)
        if output.is_err() {
            return self.list_available_runtimes_fallback();
        }

        let output = output.unwrap();

        // Parse the output to find available platforms
        // xcodebuild -downloadAllPlatforms -dry-run shows what would be downloaded
        let output_str = String::from_utf8_lossy(&output.stdout);
        let stderr_str = String::from_utf8_lossy(&output.stderr);

        // Combine stdout and stderr as xcodebuild may output to either
        let combined = format!("{}\n{}", output_str, stderr_str);

        let mut runtimes = Vec::new();
        for line in combined.lines() {
            // Look for lines mentioning iOS versions that would be downloaded
            if line.contains("iOS") && (line.contains("Downloading") || line.contains("download")) {
                if let Some(version) = Self::extract_version_from_line(line) {
                    let identifier = format!("iOS {}", version);
                    runtimes.push(Runtime::new(&identifier, &version));
                }
            }
        }

        // If no runtimes found from dry-run, use fallback
        if runtimes.is_empty() {
            return self.list_available_runtimes_fallback();
        }

        Ok(runtimes)
    }

    fn install_runtime(&self, runtime_id: &str) -> Result<()> {
        // Extract the platform name (e.g., "iOS 17.0" -> "iOS")
        // xcodebuild -downloadPlatform expects just the platform name like "iOS"
        // For specific versions, we need to use the full identifier

        println!("Downloading {}...", runtime_id);
        println!("This may take a while and require administrator privileges.");

        // Try xcodebuild -downloadPlatform first (for platform names like "iOS")
        let output = Command::new("xcodebuild")
            .args(["-downloadPlatform", runtime_id])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // If the platform name didn't work, try with xcrun simctl runtime add
            if stderr.contains("unknown platform") || stderr.contains("invalid") {
                return self.install_runtime_via_simctl(runtime_id);
            }
            return Err(Error::CommandFailed {
                command: format!("xcodebuild -downloadPlatform {}", runtime_id),
                message: stderr.to_string(),
            });
        }

        Ok(())
    }
}

// Raw serde structs for parsing xcrun simctl JSON output

#[derive(Debug, Deserialize)]
struct RawDevices {
    devices: HashMap<String, Vec<RawDevice>>,
}

#[derive(Debug, Deserialize)]
struct RawDevice {
    udid: String,
    name: String,
    state: String,
}

#[derive(Debug, Deserialize)]
struct RawRuntimes {
    runtimes: Vec<RawRuntime>,
}

#[derive(Debug, Deserialize)]
struct RawRuntime {
    identifier: String,
    version: String,
    #[serde(rename = "supportedDeviceTypes")]
    supported_device_types: Vec<RawSupportedDeviceType>,
}

#[derive(Debug, Deserialize)]
struct RawSupportedDeviceType {
    identifier: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct AvailableRuntimes {
    #[serde(default)]
    runtimes: Vec<AvailableRuntime>,
}

#[derive(Debug, Deserialize)]
struct AvailableRuntime {
    #[serde(default)]
    identifier: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    platform: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version_standard_format() {
        assert_eq!(
            IosPlatform::extract_version("com.apple.CoreSimulator.SimRuntime.iOS-17-0"),
            Some("17.0".to_string())
        );
    }

    #[test]
    fn test_extract_version_double_digit_version() {
        assert_eq!(
            IosPlatform::extract_version("com.apple.CoreSimulator.SimRuntime.iOS-16-4"),
            Some("16.4".to_string())
        );
    }

    #[test]
    fn test_extract_version_ios_15() {
        assert_eq!(
            IosPlatform::extract_version("com.apple.CoreSimulator.SimRuntime.iOS-15-5"),
            Some("15.5".to_string())
        );
    }

    #[test]
    fn test_extract_version_no_ios_prefix() {
        assert_eq!(
            IosPlatform::extract_version("com.apple.CoreSimulator.SimRuntime.watchOS-10-0"),
            None
        );
    }

    #[test]
    fn test_extract_version_empty_string() {
        assert_eq!(IosPlatform::extract_version(""), None);
    }

    #[test]
    fn test_extract_version_invalid_format() {
        assert_eq!(IosPlatform::extract_version("invalid-string"), None);
    }

    #[test]
    fn test_extract_version_incomplete_version() {
        // Only one number after iOS
        assert_eq!(IosPlatform::extract_version("iOS-17"), None);
    }

    #[test]
    fn test_extract_version_non_numeric_parts() {
        assert_eq!(IosPlatform::extract_version("iOS-abc-def"), None);
    }

    #[test]
    fn test_ios_platform_kind() {
        // Note: This test will only pass on macOS with Xcode installed
        if let Some(platform) = IosPlatform::try_new() {
            assert_eq!(platform.kind(), PlatformKind::Ios);
            assert_eq!(platform.name(), "iOS");
        }
    }

    #[test]
    fn test_check_tools_returns_status() {
        let status = IosPlatform::check_tools();
        // On non-macOS or without Xcode, should be unavailable
        // On macOS with Xcode, should be available
        // We just verify it returns a valid ToolStatus
        assert!(status.available || status.message.is_some());
    }

    #[test]
    fn test_extract_version_from_line_with_downloading() {
        assert_eq!(
            IosPlatform::extract_version_from_line("Downloading iOS 17.0 simulator runtime..."),
            Some("17.0".to_string())
        );
    }

    #[test]
    fn test_extract_version_from_line_with_download() {
        assert_eq!(
            IosPlatform::extract_version_from_line("Will download iOS 16.4"),
            Some("16.4".to_string())
        );
    }

    #[test]
    fn test_extract_version_from_line_simple() {
        assert_eq!(
            IosPlatform::extract_version_from_line("iOS 15.5 runtime"),
            Some("15.5".to_string())
        );
    }

    #[test]
    fn test_extract_version_from_line_with_trailing_chars() {
        assert_eq!(
            IosPlatform::extract_version_from_line("iOS 17.2, ready to download"),
            Some("17.2".to_string())
        );
    }

    #[test]
    fn test_extract_version_from_line_no_ios() {
        assert_eq!(
            IosPlatform::extract_version_from_line("watchOS 10.0 simulator"),
            None
        );
    }

    #[test]
    fn test_extract_version_from_line_empty() {
        assert_eq!(IosPlatform::extract_version_from_line(""), None);
    }

    #[test]
    fn test_extract_version_from_line_no_version() {
        assert_eq!(
            IosPlatform::extract_version_from_line("iOS simulator is ready"),
            None
        );
    }

    #[test]
    fn test_extract_version_from_line_ios_at_end() {
        // iOS is at the end with no version following
        assert_eq!(
            IosPlatform::extract_version_from_line("Download the iOS"),
            None
        );
    }
}
