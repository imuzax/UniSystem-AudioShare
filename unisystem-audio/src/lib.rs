#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub use linux::start_audio_capture;

#[cfg(target_os = "windows")]
pub use windows::start_audio_capture;
