use crate::domain::Device;
use crate::error::{Error, Result};
use crate::platform::{Platform, PlatformKind, PlatformRegistry};
use dialoguer::{theme::ColorfulTheme, Input, Select};

/// Prompts the user to select a platform from available options.
///
/// If only one platform is available, returns it directly without prompting.
/// If no platforms are available, returns an error.
pub fn select_platform(registry: &PlatformRegistry) -> Result<&dyn Platform> {
    let platforms = registry.platforms();

    match platforms.len() {
        0 => Err(Error::NoPlatformsAvailable),
        1 => Ok(platforms[0].as_ref()),
        _ => {
            let names: Vec<&str> = platforms.iter().map(|p| p.name()).collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select your platform")
                .default(0)
                .items(&names)
                .interact_opt()?;

            match selection {
                Some(index) => Ok(platforms[index].as_ref()),
                None => Err(Error::UserCancelled),
            }
        }
    }
}

/// Resolves a platform based on CLI flags or prompts user to select.
///
/// - If `ios` flag is set, returns the iOS platform
/// - If `android` flag is set, returns the Android platform
/// - Otherwise, prompts the user to select from available platforms
pub fn resolve_platform(
    registry: &PlatformRegistry,
    ios: bool,
    android: bool,
) -> Result<&dyn Platform> {
    if ios {
        registry
            .get(PlatformKind::Ios)
            .ok_or_else(|| Error::CommandFailed {
                command: "iOS".to_string(),
                message: "iOS platform is not available".to_string(),
            })
    } else if android {
        registry
            .get(PlatformKind::Android)
            .ok_or_else(|| Error::CommandFailed {
                command: "Android".to_string(),
                message: "Android platform is not available".to_string(),
            })
    } else {
        select_platform(registry)
    }
}

/// Prompts the user to select a device from a list.
///
/// Returns the selected device or an error if cancelled/empty.
pub fn select_device<'a>(devices: &'a [Device], prompt: &str) -> Result<&'a Device> {
    if devices.is_empty() {
        return Err(Error::NoDevicesAvailable {
            platform: "device".to_string(),
        });
    }

    let display_names: Vec<String> = devices.iter().map(|d| d.display_name()).collect();
    let display_refs: Vec<&str> = display_names.iter().map(|s| s.as_str()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&display_refs)
        .interact_opt()?;

    match selection {
        Some(index) => Ok(&devices[index]),
        None => Err(Error::UserCancelled),
    }
}

/// Generic item selection prompt.
///
/// Takes a list of items and a function to extract display names.
/// Returns the index of the selected item.
pub fn select_item<T, F>(items: &[T], prompt: &str, display_fn: F) -> Result<usize>
where
    F: Fn(&T) -> String,
{
    if items.is_empty() {
        return Err(Error::NoDevicesAvailable {
            platform: "item".to_string(),
        });
    }

    let display_names: Vec<String> = items.iter().map(&display_fn).collect();
    let display_refs: Vec<&str> = display_names.iter().map(|s| s.as_str()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&display_refs)
        .interact_opt()?;

    match selection {
        Some(index) => Ok(index),
        None => Err(Error::UserCancelled),
    }
}

/// Prompts for text input with a default value.
pub fn prompt_input(prompt: &str, default: &str) -> Result<String> {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default.to_string())
        .interact()?;
    Ok(input)
}
