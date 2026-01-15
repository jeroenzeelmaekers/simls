use crate::error::{Error, Result};
use crate::platform::PlatformRegistry;
use crate::ui::{resolve_platform, select_item};

/// Installs a new runtime (iOS simulator runtime or Android system image).
pub fn run(registry: &PlatformRegistry, ios: bool, android: bool) -> Result<()> {
    let platform = resolve_platform(registry, ios, android)?;

    // Get available runtimes (not yet installed)
    let runtimes = platform.list_available_runtimes()?;
    if runtimes.is_empty() {
        return Err(Error::NoDevicesAvailable {
            platform: format!("{} runtime", platform.name()),
        });
    }

    // Select runtime to install
    let runtime_index = select_item(&runtimes, "Select the runtime to install", |r| {
        r.name.clone()
    })?;
    let selected_runtime = &runtimes[runtime_index];

    // Install the runtime
    platform.install_runtime(&selected_runtime.identifier)?;

    println!(
        "{} runtime '{}' installed successfully.",
        platform.name(),
        selected_runtime.name
    );
    Ok(())
}
