use std::env;
use std::fs;

fn is_valid(number: isize, previous: &[isize]) -> bool {
    for i in 0 .. previous.len() {
        for j in 0 ..previous.len() {
            if previous[i] == previous[j] { continue; }
            if previous[i] + previous[j] == number {
                return true;
            }
        }
    }
     
    false
}

fn check_weakness(start_index: usize, target: isize, data: &Vec<isize>) -> (bool, isize) {
    let mut sum = 0;
    let mut values = Vec::new();
    let mut found = false;
    for i in start_index .. data.len() {
        sum += data[i];
        if sum > target {
            return (false, 0);
        }
        values.push(data[i]);
        if sum == target {
            found = true;
            break;
        }
    }

    if found {
        values.sort();
        (true, values[0] + values[values.len() - 1])
    } else {
        (false, 0)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let preamble = (&args[2]).parse::<usize>().unwrap();

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines = file_contents.lines();

    let data: Vec<isize> = lines.map(|l| l.parse::<isize>().unwrap()).collect();

    let mut index = preamble;
    let mut first_invalid = 0;
    for n in &data[preamble .. data.len()] {
        if !is_valid(*n, &data[index - preamble ..= index - 1]) {
            first_invalid = *n;
            break;
        }

        index += 1;
    }

    let mut result = 0;
    for i in 0 .. data.len() {
        let (weakness, sum) = check_weakness(i, first_invalid, &data);
        if weakness {
            println!("Weakness found at {}", i);
            result = sum;
            break;
        }
    }

    println!("Result is {}", result);
}
