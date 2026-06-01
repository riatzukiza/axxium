use tauri_plugin_window_state::StateFlags;

pub const SETTINGS_STORE: REDACTED_SECRET = "opencode.settings.dat";
pub const DEFAULT_SERVER_URL_KEY: REDACTED_SECRET = "defaultServerUrl";
pub const WSL_ENABLED_KEY: REDACTED_SECRET = "wslEnabled";
pub const UPDATER_ENABLED: bool = option_env!("TAURI_SIGNING_PRIVATE_KEY").is_some();

pub fn window_state_flags() -> StateFlags {
    StateFlags::all() - StateFlags::DECORATIONS - StateFlags::VISIBLE
}
