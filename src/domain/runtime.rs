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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_new() {
        let runtime = Runtime::new("com.apple.CoreSimulator.SimRuntime.iOS-17-0", "17.0");
        assert_eq!(
            runtime.identifier,
            "com.apple.CoreSimulator.SimRuntime.iOS-17-0"
        );
        assert_eq!(runtime.name, "17.0");
        assert!(runtime.supported_device_types.is_empty());
    }

    #[test]
    fn test_runtime_with_device_types() {
        let device_types = vec![
            DeviceType::new("iphone15", "iPhone 15"),
            DeviceType::new("iphone15pro", "iPhone 15 Pro"),
        ];
        let runtime = Runtime::new("runtime-id", "17.0").with_device_types(device_types);

        assert_eq!(runtime.supported_device_types.len(), 2);
        assert_eq!(runtime.supported_device_types[0].name, "iPhone 15");
        assert_eq!(runtime.supported_device_types[1].name, "iPhone 15 Pro");
    }

    #[test]
    fn test_runtime_with_empty_device_types() {
        let runtime = Runtime::new("runtime-id", "17.0").with_device_types(vec![]);
        assert!(runtime.supported_device_types.is_empty());
    }

    #[test]
    fn test_device_type_new() {
        let dt = DeviceType::new(
            "com.apple.CoreSimulator.SimDeviceType.iPhone-15",
            "iPhone 15",
        );
        assert_eq!(
            dt.identifier,
            "com.apple.CoreSimulator.SimDeviceType.iPhone-15"
        );
        assert_eq!(dt.name, "iPhone 15");
    }

    #[test]
    fn test_device_type_with_string_inputs() {
        let dt = DeviceType::new(String::from("pixel_7"), String::from("Pixel 7"));
        assert_eq!(dt.identifier, "pixel_7");
        assert_eq!(dt.name, "Pixel 7");
    }

    #[test]
    fn test_runtime_builder_chain() {
        let runtime = Runtime::new("android-34", "Android 14")
            .with_device_types(vec![DeviceType::new("pixel_7", "Pixel 7")]);

        assert_eq!(runtime.identifier, "android-34");
        assert_eq!(runtime.name, "Android 14");
        assert_eq!(runtime.supported_device_types.len(), 1);
    }
}
