use std::time::Duration;
use std::thread;
use std::path::Path;

const ERROR_MSG: &'static str = "Failed to generate metadata files";

fn main() {
	// Allow run time to be built in the background.
	// Theoretically faster builds if dependencies in the node
	// cargo are linked right.
	while !Path::new("/home/travis/substrate-node-joystream/substrate-runtime-joystream/is_built").exists() {
		thread::sleep(Duration::from_millis(10000));
		println!("Waiting for runtime to be built...");
	}
}
