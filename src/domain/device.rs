/// Unified device representation for both iOS Simulators and Android Emulators.
/// This provides a platform-agnostic interface for device operations (LSP compliance).
#[derive(Debug, Clone)]
pub struct Device {
    /// Unique identifier for the device (UDID for iOS, name for Android)
    pub id: String,
    /// Display name of the device
    pub name: String,
    /// Current state of the device
    pub state: DeviceState,
    /// Platform-specific version info (e.g., "iOS 17.0" or "Android 14")
    pub platform_version: Option<String>,
}

impl Device {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            state: DeviceState::Unknown,
            platform_version: None,
        }
    }

    pub fn with_state(mut self, state: DeviceState) -> Self {
        self.state = state;
        self
    }

    pub fn with_platform_version(mut self, version: impl Into<String>) -> Self {
        self.platform_version = Some(version.into());
        self
    }

    /// Returns a display name including platform version if available
    pub fn display_name(&self) -> String {
        match &self.platform_version {
            Some(version) => format!("{} ({})", self.name, version),
            None => self.name.clone(),
        }
    }
}

/// Device state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeviceState {
    Booted,
    Shutdown,
    #[default]
    Unknown,
}

impl DeviceState {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceState::Booted => "Booted",
            DeviceState::Shutdown => "Shutdown",
            DeviceState::Unknown => "Unknown",
        }
    }
}

impl From<&str> for DeviceState {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "booted" => DeviceState::Booted,
            "shutdown" => DeviceState::Shutdown,
            _ => DeviceState::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_new() {
        let device = Device::new("test-id", "Test Device");
        assert_eq!(device.id, "test-id");
        assert_eq!(device.name, "Test Device");
        assert_eq!(device.state, DeviceState::Unknown);
        assert!(device.platform_version.is_none());
    }

    #[test]
    fn test_device_with_state() {
        let device = Device::new("id", "name").with_state(DeviceState::Booted);
        assert_eq!(device.state, DeviceState::Booted);
    }

    #[test]
    fn test_device_with_platform_version() {
        let device = Device::new("id", "name").with_platform_version("17.0");
        assert_eq!(device.platform_version, Some("17.0".to_string()));
    }

    #[test]
    fn test_device_display_name_without_version() {
        let device = Device::new("id", "iPhone 15");
        assert_eq!(device.display_name(), "iPhone 15");
    }

    #[test]
    fn test_device_display_name_with_version() {
        let device = Device::new("id", "iPhone 15").with_platform_version("17.0");
        assert_eq!(device.display_name(), "iPhone 15 (17.0)");
    }

    #[test]
    fn test_device_builder_chain() {
        let device = Device::new("udid-123", "iPhone 15 Pro")
            .with_state(DeviceState::Shutdown)
            .with_platform_version("17.2");

        assert_eq!(device.id, "udid-123");
        assert_eq!(device.name, "iPhone 15 Pro");
        assert_eq!(device.state, DeviceState::Shutdown);
        assert_eq!(device.platform_version, Some("17.2".to_string()));
    }

    #[test]
    fn test_device_state_default() {
        let state = DeviceState::default();
        assert_eq!(state, DeviceState::Unknown);
    }

    #[test]
    fn test_device_state_as_str() {
        assert_eq!(DeviceState::Booted.as_str(), "Booted");
        assert_eq!(DeviceState::Shutdown.as_str(), "Shutdown");
        assert_eq!(DeviceState::Unknown.as_str(), "Unknown");
    }

    #[test]
    fn test_device_state_from_str_booted() {
        assert_eq!(DeviceState::from("booted"), DeviceState::Booted);
        assert_eq!(DeviceState::from("Booted"), DeviceState::Booted);
        assert_eq!(DeviceState::from("BOOTED"), DeviceState::Booted);
    }

    #[test]
    fn test_device_state_from_str_shutdown() {
        assert_eq!(DeviceState::from("shutdown"), DeviceState::Shutdown);
        assert_eq!(DeviceState::from("Shutdown"), DeviceState::Shutdown);
        assert_eq!(DeviceState::from("SHUTDOWN"), DeviceState::Shutdown);
    }

    #[test]
    fn test_device_state_from_str_unknown() {
        assert_eq!(DeviceState::from(""), DeviceState::Unknown);
        assert_eq!(DeviceState::from("invalid"), DeviceState::Unknown);
        assert_eq!(DeviceState::from("starting"), DeviceState::Unknown);
    }
}
