/// Represents a runtime/system image that devices can be created with.
/// For iOS: Runtime versions (e.g., iOS 17.0)
/// For Android: System images (e.g., system-images;android-34;google_apis;arm64-v8a)
#[derive(Debug, Clone)]
pub struct Runtime {
    /// Unique identifier for the runtime
    pub identifier: String,
    /// Human-readable name/version
    pub name: String,
    /// Supported device types for this runtime
    pub supported_device_types: Vec<DeviceType>,
}

impl Runtime {
    pub fn new(identifier: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
            name: name.into(),
            supported_device_types: Vec::new(),
        }
    }

    pub fn with_device_types(mut self, types: Vec<DeviceType>) -> Self {
        self.supported_device_types = types;
        self
    }
}

/// Represents a device type/profile that can be used to create a new device.
/// For iOS: Device types like "iPhone 15", "iPad Pro"
/// For Android: Device profiles like "pixel_7", "Nexus 5"
#[derive(Debug, Clone)]
pub struct DeviceType {
    /// Unique identifier for the device type
    pub identifier: String,
    /// Human-readable name
    pub name: String,
}

impl DeviceType {
    pub fn new(identifier: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
            name: name.into(),
        }
    }
}
