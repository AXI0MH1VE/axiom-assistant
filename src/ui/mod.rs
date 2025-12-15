#[cfg(feature = "ui")]
pub mod tauri_app;

#[cfg(feature = "ui")]
pub use tauri_app::init_tauri;
