#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
#[macro_use]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;
