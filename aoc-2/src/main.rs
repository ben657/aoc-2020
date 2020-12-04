use std::env;
use std::fs;
use regex::Regex;
use regex::Captures;

struct Policy {
	character: char,
	i1: usize,
	i2: usize
}

struct Password {
	policy: Policy,
	text: String
}

fn parse_password_line(line: &str) -> Password {
	let re = Regex::new(r"(\d+)-(\d+) ([A-z]): (.*)").unwrap();
	let captures: Vec<Captures> = re.captures_iter(line).collect();
	let capture = &captures[0];
	
	let i1 = capture[1].parse::<usize>().expect("");
	let i2 = capture[2].parse::<usize>().expect("");
	let character = &capture[3].chars().next().unwrap();
	let text = String::from(&capture[4]);
	Password {
		policy: Policy {
			i1,
			i2,
			character: *character
		},
		text
	}
}

fn validate_password(password: &Password) -> bool {
	let mut required_char_count = 0;
	for (i, c) in password.text.chars().enumerate() {
		if (i + 1 == password.policy.i1 || i + 1 == password.policy.i2) && c == password.policy.character {
			required_char_count += 1;
		}
	}

	required_char_count == 1
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
	let lines = file_contents.lines();

	let passwords = lines.map(|l| parse_password_line(l));
	let mut valid_count = 0;
	for password in passwords {
		if validate_password(&password) {
			valid_count += 1;
		}
	}

	println!("{} valid passwords", valid_count);
}
