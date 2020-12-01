use std::env;
use std::fs;

fn main() {
		let args: Vec<String> = env::args().collect();

		let target = &args[1];
		let target_int = target.parse::<i32>().expect("Unable to parse target, must be an int");

		let filename = &args[2];

		let file_contents = fs::read_to_string(filename).expect("Error reading file");
		let lines = file_contents.lines();
		let numbers: Vec<i32> = lines.map(|l| l.parse::<i32>().expect("")).collect();

		let mut r1 = 0;
		let mut r2 = 0;

		for &n in numbers.iter() {
			if n == target_int {
				r1 = n;
				break;
			}
			if n > target_int {
				continue;
			}

			let mut found = false;

			for &n2 in numbers.iter() {
				if n + n2 == target_int {
					r1 = n;
					r2 = n2;
					found = true;
					break;
				}
			}

			if found {break;}
		}

		println!("{} x {} = {}", r1, r2, r1 * r2);
}
