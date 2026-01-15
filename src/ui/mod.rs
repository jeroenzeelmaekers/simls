mod selection;

pub use selection::{prompt_input, resolve_platform, select_device, select_item};

// Note: select_platform is available but typically resolve_platform is preferred
// as it handles CLI flag logic automatically.
