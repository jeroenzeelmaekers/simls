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
