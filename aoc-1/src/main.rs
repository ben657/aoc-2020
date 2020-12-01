use std::env;
use std::fs;

fn main() {
		let args: Vec<String> = env::args().collect();

		let target = &args[1];
		let filename = &args[2];

		let file_contents = fs::read_to_string(filename).expect("Error reading file");
		let lines: Vec<&str> = file_contents.split("\n").collect();
		
}
