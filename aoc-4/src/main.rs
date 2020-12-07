use std::env;
use std::fs;
use std::collections::HashMap;

struct Key {
    text: String,
    validator: fn(&String) -> bool
}

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
        Key {
            text: String::from("byr"),
            validator: |value| {
                let v_int = value.parse::<usize>().unwrap();
                v_int <= 2002 && v_int >= 1920
            }
        },
        Key {
            text: String::from("iyr"),
            validator: |value| {
                let v_int = value.parse::<usize>().unwrap();
                v_int <= 2020 && v_int >= 2010
            }
        },
        Key {
            text: String::from("eyr"),
            validator: |value| {
                let v_int = value.parse::<usize>().unwrap();
                v_int <= 2030 && v_int >= 2020
            }
        },
        Key {
            text: String::from("hgt"),
            validator: |value| {
                let chars: Vec<char> = value.chars().collect();
                let units = String::from(chars[chars.len() - 2]) + &String::from(chars[chars.len() - 1]);
                if units != "cm" && units != "in" {
                    return false;
                }
                let mut number_str = String::new();
                for i in 0 .. chars.len() - 2 {
                    number_str += &String::from(chars[i]);
                }
                let number = number_str.parse::<usize>().unwrap();
                if units == "cm" {
                    number >= 150 && number <= 193
                } else if units == "in" {
                    number >= 59 && number <= 76
                } else {
                    false
                }
            }
        },
        Key {
            text: String::from("hcl"),
            validator: |value| {
                let chars: Vec<char> = value.chars().collect();
                if chars[0] != '#' { 
                    false
                } else {
                    let mut valid = true;
                    for i in 1 .. chars.len() {
                        let c = chars[i];
                        if !c.is_ascii_hexdigit() {
                            valid = false;
                            break;
                        }
                    }
                    valid
                }
            }
        },
        Key {
            text: String::from("ecl"),
            validator: |value| {
                let valid_values = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                valid_values.contains(&value.as_str())
            }
        },
        Key {
            text: String::from("pid"),
            validator: |value| {
                value.len() == 9
            }
        }
        // String::from("byr"),
        // String::from("iyr"),
        // String::from("eyr"),
        // String::from("hgt"),
        // String::from("hcl"),
        // String::from("ecl"),
        // String::from("pid")
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
            println!("{}",key.text);
            if !passport.contains_key(&key.text) {
                valid = false;
                break;
            } else if !((key.validator)(&passport[&key.text])) {
                println!("{}:{} valid: {}", key.text, passport[&key.text], false);
                valid = false;
                break;
            }
        }
        if valid { valid_count += 1; }
    }

    println!("There are {} valid passports", valid_count);
}
