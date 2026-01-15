use crate::error::{Error, Result};
use crate::platform::PlatformRegistry;
use crate::ui::{prompt_input, resolve_platform, select_item};

/// Creates a new device on the chosen platform.
pub fn run(registry: &PlatformRegistry, ios: bool, android: bool) -> Result<()> {
    let platform = resolve_platform(registry, ios, android)?;

    // Get available runtimes
    let runtimes = platform.list_runtimes()?;
    if runtimes.is_empty() {
        return Err(Error::NoDevicesAvailable {
            platform: format!("{} runtime", platform.name()),
        });
    }

    // Select runtime
    let runtime_index = select_item(&runtimes, "Select your runtime", |r| r.name.clone())?;
    let selected_runtime = &runtimes[runtime_index];

    // Get device types - for iOS use the runtime's supported types, for Android fetch separately
    let device_types = if selected_runtime.supported_device_types.is_empty() {
        platform.list_device_types()?
    } else {
        selected_runtime.supported_device_types.clone()
    };

    if device_types.is_empty() {
        return Err(Error::NoDevicesAvailable {
            platform: format!("{} device type", platform.name()),
        });
    }

    // Select device type
    let device_type_index = select_item(&device_types, "Select your device type", |dt| {
        dt.name.replace(' ', "_")
    })?;
    let selected_device_type = &device_types[device_type_index];

    // Get device name
    let default_name = format!(
        "{} {}",
        selected_device_type.name.replace(' ', "_"),
        selected_runtime.name
    );
    let device_name = prompt_input("Enter the device name", &default_name)?;

    // Create the device
    platform.create_device(
        &device_name,
        &selected_device_type.identifier,
        &selected_runtime.identifier,
    )?;

    println!(
        "{} device '{}' created successfully.",
        platform.name(),
        device_name
    );
    Ok(())
}
