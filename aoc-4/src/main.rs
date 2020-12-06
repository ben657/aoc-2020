use std::env;
use std::fs;
use std::collections::HashMap;

fn parse_passport_string(s: &String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let pairs = s.trim().split(" ");
    for pair in pairs {
        let parts: Vec<&str> = pair.split(":").collect();
        let key = String::from(parts[0]);
        let value = String::from(parts[1]);
        map.insert(key, value);
    }

    map
}

fn main() {
    let required_keys = [
        String::from("byr"),
        String::from("iyr"),
        String::from("eyr"),
        String::from("hgt"),
        String::from("hcl"),
        String::from("ecl"),
        String::from("pid")
    ];

    let args: Vec<String> = env::args().collect();

	let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines = file_contents.lines();
    
    let mut passports = Vec::new();
    let mut current_passport_string = String::from("");
    for line in lines {
        if line.len() == 0 && current_passport_string != "" {
            let passport = parse_passport_string(&current_passport_string);
            passports.push(passport);
            current_passport_string = String::from("");
        } else {
            current_passport_string += " ";
            current_passport_string += line;
        }
    }
    let passport = parse_passport_string(&current_passport_string);
    passports.push(passport);

    let mut valid_count = 0;
    for passport in passports {
        let mut valid = true;
        for key in required_keys.iter() {
            if !passport.contains_key(key) {
                valid = false;
                break;
            }
        }
        if valid { valid_count += 1; }
    }

    println!("There are {} valid passports", valid_count);
}
