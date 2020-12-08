use std::env;
use std::fs;
use std::collections::HashSet;

fn main() {
	let args: Vec<String> = env::args().collect();

	let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
	let lines = file_contents.lines();
	
	let mut groups: Vec<Vec<String>> = Vec::new();

	let mut last_line = "";
	let mut current_group_idx: isize = -1;
	for line in lines {
		if last_line.len() == 0 {
			groups.push(Vec::new());
			current_group_idx += 1;
		}

		if line.len() > 0 {
			groups[current_group_idx as usize].push(String::from(line));
		}

		last_line = line;
	}

	let mut unique_answers_per_group = Vec::new();

	for group in groups.iter() {
		let mut all_answers: Vec<HashSet<char>> = Vec::new();
		for person in group.iter() {
			let mut answers = HashSet::new();
			for c in person.chars() {
				answers.insert(c);
			}
			all_answers.push(answers);
		}
		let result: HashSet<&char> = all_answers[0];
		for i in 1 .. all_answers.len() {
			result = result.intersection(&all_answers[i]).collect();
		}
		unique_answers_per_group.push(answers.len());
	}

	let mut sum = 0;
	let mut output = String::new();
	for count in unique_answers_per_group {
		sum += count;
		output += &format!("{} + ", count);
	}

	println!("{} = {}", &output[..output.len() - 3], sum);
}
