use crate::domain::{Device, DeviceType, Runtime};
use crate::error::{Error, Result};
use crate::platform::{Platform, PlatformKind, ToolStatus};
use std::process::Command;

/// Android Platform implementation using Android SDK tools
pub struct AndroidPlatform {
    _private: (), // Prevent external construction
}

impl AndroidPlatform {
    /// Attempts to create an Android platform instance.
    /// Returns None if Android tools are not available.
    pub fn try_new() -> Option<Self> {
        let status = Self::check_tools();
        if status.available {
            Some(Self { _private: () })
        } else {
            None
        }
    }

    /// Check if Android development tools are available
    pub fn check_tools() -> ToolStatus {
        if !cfg!(target_os = "macos") {
            return ToolStatus::unavailable(
                "Android emulators are currently only supported on macOS.",
            );
        }

        // Check if emulator is available
        let emulator_exists = Command::new("emulator").args(["-version"]).output().is_ok();
        if !emulator_exists {
            return ToolStatus::unavailable(
                "Android Emulator not found. \
                 Make sure Android SDK is installed and ANDROID_HOME is set. \
                 Add $ANDROID_HOME/emulator to your PATH.",
            );
        }

        // Check if avdmanager is available
        let avdmanager_exists = Command::new("avdmanager")
            .args(["list", "target"])
            .output()
            .is_ok();
        if !avdmanager_exists {
            return ToolStatus::unavailable(
                "Android AVD Manager not found. \
                 Make sure Android SDK is installed and ANDROID_HOME is set. \
                 Add $ANDROID_HOME/cmdline-tools/latest/bin to your PATH.",
            );
        }

        ToolStatus::available()
    }

    /// Parse system image path from sdkmanager output line
    fn parse_system_image(line: &str) -> Option<String> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 4 {
            let path_parts: Vec<&str> = parts[0].trim().split(';').collect();
            if path_parts.len() == 4 {
                return Some(format!(
                    "system-images;{};{};{}",
                    path_parts[1].trim(),
                    path_parts[2].trim(),
                    path_parts[3].trim()
                ));
            }
        }
        None
    }

    /// Format a system image path into a human-readable name
    /// e.g., "system-images;android-34;google_apis;arm64-v8a" -> "Android 34 (Google APIs, arm64-v8a)"
    fn format_system_image_name(path: &str) -> String {
        let parts: Vec<&str> = path.split(';').collect();
        if parts.len() == 4 {
            let api_level = parts[1].replace("android-", "Android ");
            let variant = parts[2].replace('_', " ");
            let arch = parts[3];
            format!("{} ({}, {})", api_level, variant, arch)
        } else {
            path.to_string()
        }
    }
}

impl Platform for AndroidPlatform {
    fn kind(&self) -> PlatformKind {
        PlatformKind::Android
    }

    fn list_devices(&self) -> Result<Vec<Device>> {
        let output = Command::new("emulator").args(["-list-avds"]).output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "emulator -list-avds".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let output_string = String::from_utf8(output.stdout).map_err(|e| Error::Parse {
            message: format!("Invalid UTF-8 in command output: {}", e),
        })?;

        let devices = output_string
            .lines()
            .filter(|line| !line.is_empty())
            .map(|name| Device::new(name, name))
            .collect();

        Ok(devices)
    }

    fn start_device(&self, device_id: &str) -> Result<()> {
        let output = Command::new("screen")
            .args(["-m", "-d", "emulator", "-avd", device_id])
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: format!("emulator -avd {}", device_id),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Ok(())
    }

    fn create_device(&self, name: &str, device_type: &str, runtime: &str) -> Result<()> {
        let output = Command::new("avdmanager")
            .args([
                "create",
                "avd",
                "-n",
                name,
                "-k",
                runtime,
                "-d",
                device_type,
            ])
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "avdmanager create avd".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Ok(())
    }

    fn delete_device(&self, device_id: &str) -> Result<()> {
        let output = Command::new("avdmanager")
            .args(["delete", "avd", "--name", device_id])
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "avdmanager delete avd".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Ok(())
    }

    fn erase_device(&self, _device_id: &str) -> Result<()> {
        Err(Error::NotImplemented {
            feature: "Android emulator erase".to_string(),
        })
    }

    fn list_runtimes(&self) -> Result<Vec<Runtime>> {
        let output = Command::new("sdkmanager").args(["--list"]).output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "sdkmanager --list".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let output_str = String::from_utf8_lossy(&output.stdout);

        let runtimes: Vec<Runtime> = output_str
            .lines()
            .filter(|line| line.contains("system-images;"))
            .filter_map(Self::parse_system_image)
            .map(|path| Runtime::new(&path, &path))
            .collect();

        if runtimes.is_empty() {
            return Err(Error::NoDevicesAvailable {
                platform: "Android system image".to_string(),
            });
        }

        Ok(runtimes)
    }

    fn list_device_types(&self) -> Result<Vec<DeviceType>> {
        let output = Command::new("avdmanager")
            .args(["list", "device"])
            .output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "avdmanager list device".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let output_str = String::from_utf8_lossy(&output.stdout);

        let mut device_types = Vec::new();
        for line in output_str.lines() {
            if line.starts_with("id:") {
                if let Some(name) = line.split('"').nth(1) {
                    device_types.push(DeviceType::new(name, name));
                }
            }
        }

        if device_types.is_empty() {
            return Err(Error::NoDevicesAvailable {
                platform: "Android device profile".to_string(),
            });
        }

        Ok(device_types)
    }

    fn list_available_runtimes(&self) -> Result<Vec<Runtime>> {
        // Use sdkmanager --list to get all available system images (including not installed)
        let output = Command::new("sdkmanager").args(["--list"]).output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: "sdkmanager --list".to_string(),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let output_str = String::from_utf8_lossy(&output.stdout);

        // Get installed runtimes to filter them out
        let installed = self.list_runtimes().unwrap_or_default();
        let installed_ids: std::collections::HashSet<_> =
            installed.iter().map(|r| r.identifier.clone()).collect();

        let runtimes: Vec<Runtime> = output_str
            .lines()
            .filter(|line| line.contains("system-images;"))
            .filter_map(Self::parse_system_image)
            .filter(|path| !installed_ids.contains(path))
            .map(|path| {
                let display_name = Self::format_system_image_name(&path);
                Runtime::new(&path, &display_name)
            })
            .collect();

        if runtimes.is_empty() {
            return Err(Error::NoDevicesAvailable {
                platform: "Android system image".to_string(),
            });
        }

        Ok(runtimes)
    }

    fn install_runtime(&self, runtime_id: &str) -> Result<()> {
        println!("Installing {}...", runtime_id);
        println!("This may take a while depending on your internet connection.");

        // Use sdkmanager to install the system image
        // The --install flag is optional, we can just pass the package name
        let output = Command::new("sdkmanager").args([runtime_id]).output()?;

        if !output.status.success() {
            return Err(Error::CommandFailed {
                command: format!("sdkmanager {}", runtime_id),
                message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        println!("Successfully installed {}", runtime_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_system_image_valid() {
        let line = "system-images;android-34;google_apis;arm64-v8a | 14 | Google APIs ARM 64 v8a | system-images/android-34/google_apis/arm64-v8a";
        let result = AndroidPlatform::parse_system_image(line);
        assert_eq!(
            result,
            Some("system-images;android-34;google_apis;arm64-v8a".to_string())
        );
    }

    #[test]
    fn test_parse_system_image_android_33() {
        let line = "system-images;android-33;google_apis_playstore;x86_64 | 33 | Google Play x86_64 | system-images/android-33/google_apis_playstore/x86_64";
        let result = AndroidPlatform::parse_system_image(line);
        assert_eq!(
            result,
            Some("system-images;android-33;google_apis_playstore;x86_64".to_string())
        );
    }

    #[test]
    fn test_parse_system_image_invalid_format() {
        // Missing parts
        let line = "system-images;android-34";
        let result = AndroidPlatform::parse_system_image(line);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_system_image_empty_string() {
        let result = AndroidPlatform::parse_system_image("");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_system_image_not_system_image() {
        let line = "platforms;android-34 | 34 | Android API 34";
        let result = AndroidPlatform::parse_system_image(line);
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_system_image_fewer_than_four_pipes() {
        let line = "system-images;android-34;google_apis;arm64-v8a | 14 | Description";
        let result = AndroidPlatform::parse_system_image(line);
        assert_eq!(result, None);
    }

    #[test]
    fn test_android_platform_kind() {
        // Note: This test will only pass on macOS with Android SDK installed
        if let Some(platform) = AndroidPlatform::try_new() {
            assert_eq!(platform.kind(), PlatformKind::Android);
            assert_eq!(platform.name(), "Android");
        }
    }

    #[test]
    fn test_check_tools_returns_status() {
        let status = AndroidPlatform::check_tools();
        // On non-macOS or without Android SDK, should be unavailable
        // On macOS with Android SDK, should be available
        // We just verify it returns a valid ToolStatus
        assert!(status.available || status.message.is_some());
    }

    #[test]
    fn test_format_system_image_name_standard() {
        assert_eq!(
            AndroidPlatform::format_system_image_name(
                "system-images;android-34;google_apis;arm64-v8a"
            ),
            "Android 34 (google apis, arm64-v8a)"
        );
    }

    #[test]
    fn test_format_system_image_name_with_playstore() {
        assert_eq!(
            AndroidPlatform::format_system_image_name(
                "system-images;android-33;google_apis_playstore;x86_64"
            ),
            "Android 33 (google apis playstore, x86_64)"
        );
    }

    #[test]
    fn test_format_system_image_name_default_variant() {
        assert_eq!(
            AndroidPlatform::format_system_image_name("system-images;android-30;default;x86"),
            "Android 30 (default, x86)"
        );
    }

    #[test]
    fn test_format_system_image_name_invalid_format() {
        // Should return the original path if format is unexpected
        assert_eq!(
            AndroidPlatform::format_system_image_name("invalid-path"),
            "invalid-path"
        );
    }

    #[test]
    fn test_format_system_image_name_too_few_parts() {
        assert_eq!(
            AndroidPlatform::format_system_image_name("system-images;android-34"),
            "system-images;android-34"
        );
    }

    #[test]
    fn test_format_system_image_name_empty() {
        assert_eq!(AndroidPlatform::format_system_image_name(""), "");
    }
}
