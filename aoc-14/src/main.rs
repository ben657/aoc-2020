use std::env;
use std::fs;
use std::collections::HashMap;

fn num_to_bin(num: isize) -> String {
    let mut result = format!("{:b}", num);
    let pad = 36 - result.len();
    for _ in 0 .. pad {
        result = String::from("0") + &result;
    }

    result
}

fn bin_to_num(bin: &str) -> isize {
    isize::from_str_radix(bin, 2).expect("Could not turn binary string into number")
}

#[derive(Debug)]
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

#[derive(Debug)]
enum Instruction {
    MaskSet(Mask),
    MemSet(isize, isize)
}

fn main() {
    let args: Vec<String> = env::args().collect();

	let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines: Vec<&str> = file_contents.lines().collect();

    let mut instructions = Vec::new();

    for line in lines.iter() {
        if &line[..4] == "mask" {
            let mask_str = (&line[7..]).to_owned();
            let mask = Mask::from(&mask_str);
            instructions.push(Instruction::MaskSet(mask));
        } else {
            let mut sqbrt_end = 0;
            for (i, c) in line[4..].chars().enumerate() {
                if c == ']' {
                    sqbrt_end = i + 4;
                    break;
                }
            }

            let mem_addr = line[4 .. sqbrt_end].parse::<isize>().unwrap();
            let mem_val = line[sqbrt_end + 4 ..].parse::<isize>().unwrap();
            instructions.push(Instruction::MemSet(mem_addr, mem_val));
        }
    }

    let mut mask = Mask::from("X");
    let mut memory: HashMap<isize, isize> = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::MaskSet(new_mask) => mask = new_mask,
            Instruction::MemSet(addr, val) => { 
                let masked_val = mask.apply(val);
                memory.entry(addr).and_modify(|v| *v = masked_val).or_insert(masked_val); 
            }
        }
    }

    let mut result = 0;
    for (_, v) in memory.iter() {
        result += v;
    }
    println!("{}", result);
}