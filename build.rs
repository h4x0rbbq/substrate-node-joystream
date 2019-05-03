use vergen::{ConstantsFlags, generate_cargo_keys};
use std::time::Duration;
use std::thread;
use std::path::Path;

const ERROR_MSG: &'static str = "Failed to generate metadata files";

fn main() {
	generate_cargo_keys(ConstantsFlags::all()).expect(ERROR_MSG);
	println!("cargo:rerun-if-changed=.git/HEAD");
	
	// Allow run time to be built in the background.
	// Theoretically faster builds if dependencies in the node
	// cargo are linked right.
	while !Path::new("substrate-runtime-joystream/is_built").exists() {
		thread::sleep(Duration::from_millis(1000));
	}
}
