use std::env;
use std::fs;

fn num_to_bin(num: isize) -> String {
    format!("{:b}", num)
}

fn bin_to_num(bin: &str) -> isize {
    isize::from_str_radix(bin, 2).expect("Could not turn binary string into number")
}

struct Mask {
    mask: Vec<char>
}

impl Mask {
    fn from(s: &str) -> Mask {
        Mask {
            mask: s.chars().collect()
        }
    }
    
    fn apply(&self, input: isize) -> isize {
        let mut bin_string = num_to_bin(input);
        for i in (0 .. self.mask.len()).rev() {
            if self.mask[i] != 'X' { 
                let mut bytes = bin_string.clone().into_bytes();
                bytes[i] = self.mask[i] as u8;
                bin_string = String::from_utf8(bytes).unwrap();
            }
        }
        bin_to_num(&bin_string)
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();

	// let filename = &args[1];

	// let file_contents = fs::read_to_string(filename).expect("Error reading file");
    // let lines: Vec<&str> = file_contents.lines().collect();

    let mask = Mask::from("0X1");
    println!("{}", mask.apply(4));
}