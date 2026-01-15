use crate::error::{Error, Result};
use crate::platform::PlatformRegistry;
use crate::ui::{resolve_platform, select_device};

/// Erases all content and settings from a selected device.
pub fn run(registry: &PlatformRegistry, ios: bool, android: bool) -> Result<()> {
    let platform = resolve_platform(registry, ios, android)?;
    let devices = platform.list_devices()?;

    if devices.is_empty() {
        return Err(Error::NoDevicesAvailable {
            platform: platform.name().to_string(),
        });
    }

    let selected = select_device(
        &devices,
        "Select the device to erase content and settings from",
    )?;

    platform.erase_device(&selected.id)?;

    println!(
        "{} device '{}' erased successfully.",
        platform.name(),
        selected.name
    );
    Ok(())
}
