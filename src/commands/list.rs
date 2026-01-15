use crate::domain::DeviceState;
use crate::error::Result;
use crate::platform::{Platform, PlatformRegistry};
use colored::Colorize;

/// Lists all devices for available platforms.
///
/// If specific platform flags are provided, only lists that platform.
/// Otherwise lists all available platforms.
pub fn run(registry: &PlatformRegistry, ios: bool, android: bool) -> Result<()> {
    let platforms_to_list: Vec<&dyn Platform> = if ios || android {
        registry
            .platforms()
            .iter()
            .filter(|p| {
                (ios && p.kind() == crate::platform::PlatformKind::Ios)
                    || (android && p.kind() == crate::platform::PlatformKind::Android)
            })
            .map(|p| p.as_ref())
            .collect()
    } else {
        registry.platforms().iter().map(|p| p.as_ref()).collect()
    };

    for platform in platforms_to_list {
        println!("{} Devices:", platform.name());

        match platform.list_devices() {
            Ok(devices) => {
                if devices.is_empty() {
                    println!("  No {} devices found.", platform.name().to_lowercase());
                } else {
                    for device in devices {
                        let status = match device.state {
                            DeviceState::Booted => device.state.as_str().green(),
                            DeviceState::Shutdown => device.state.as_str().red(),
                            DeviceState::Unknown => device.state.as_str().normal(),
                        };
                        println!("  {} - {}", device.display_name(), status);
                    }
                }
            }
            Err(e) => {
                println!("  Error listing devices: {}", e);
            }
        }
    }

    Ok(())
}
