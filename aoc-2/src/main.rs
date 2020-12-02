use std::env;
use std::fs;
use regex::Regex;
use regex::Captures;

struct Policy {
	character: String,
	min: isize,
	max: isize
}

struct Password {
	policy: Policy,
	text: String
}

fn ParsePasswordLine(line: &String) {
	// let sections: Vec<&str> = line.split(' ').collect();

	// let bounds = sections[0].split('-');
	// let 
	let re = Regex::new(r"(\d+)-(\d+) ([A-z]): (.*)").unwrap();
	let captures: Vec<regex::Captures> = re.captures_iter(line).collect();
	let capture = &captures[0];
	
	let min = capture[0].parse::<isize>().expect("");
	let max = capture[1].parse::<isize>().expect("");
	let character = String::from(&capture[2]);
	let text = String::from(&capture[3]);

	Password {
		policy: Policy {
			min,
			max,
			character
		},
		text
	};
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
	let lines = file_contents.lines();

	lines.map(|l| ParsePasswordLine(l))
}
