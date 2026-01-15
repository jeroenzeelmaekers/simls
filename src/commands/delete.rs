use crate::error::{Error, Result};
use crate::platform::PlatformRegistry;
use crate::ui::{resolve_platform, select_device};

/// Deletes a selected device on the chosen platform.
pub fn run(registry: &PlatformRegistry, ios: bool, android: bool) -> Result<()> {
    let platform = resolve_platform(registry, ios, android)?;
    let devices = platform.list_devices()?;

    if devices.is_empty() {
        return Err(Error::NoDevicesAvailable {
            platform: platform.name().to_string(),
        });
    }

    let selected = select_device(&devices, "Select the device to delete")?;

    platform.delete_device(&selected.id)?;

    println!(
        "{} device '{}' deleted successfully.",
        platform.name(),
        selected.name
    );
    Ok(())
}
