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
