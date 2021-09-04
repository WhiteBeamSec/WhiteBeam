pub mod platforms;
#[cfg(target_os = "windows")]
use platforms::windows as platform;
#[cfg(target_os = "linux")]
use platforms::linux as platform;
#[cfg(target_os = "macos")]
use platforms::macos as platform;

fn main() {
    platform::run_tests();
}
