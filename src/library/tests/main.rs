use std::env;

pub mod platforms;
#[cfg(target_os = "windows")]
use platforms::windows as platform;
#[cfg(target_os = "linux")]
use platforms::linux as platform;
#[cfg(target_os = "macos")]
use platforms::macos as platform;

fn main() {
	let args: Vec<String> = env::args().collect();
	if (args.len()-1) > 1 {
		let test = &args[1].to_lowercase();
		let test_type = &args[2].to_lowercase();
		platform::run_test(test, test_type);
	} else {
		eprintln!("WhiteBeam: No test or test type provided");
	}
}
