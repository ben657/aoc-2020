use std::fs;
use std::env;
use std::collections::HashSet;

struct Instruction {
    op: String,
    arg: isize,
    visited: bool
}

fn parse_instruction_line(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(' ').collect();
    
    Instruction {
        op: String::from(parts[0]),
        arg: parts[1].parse::<isize>().expect("Failed to parse arg"),
        visited: false
    }
}

fn run(instructions: &mut Vec<Instruction>) -> (isize, bool) {
    let mut current_index: isize = 0;
    let mut accumulator = 0;

    let mut is_loop = false;

    for i in instructions.iter_mut() {
        i.visited = false;
    }

    while current_index < instructions.len() as isize {
        let mut instruction = &mut instructions[current_index as usize];
        if instruction.visited { 
            is_loop = true;
            break;
        }
        instruction.visited = true;
        current_index += 1;

        match instruction.op.as_str() {
            "acc" => accumulator += instruction.arg,
            "jmp" => current_index += instruction.arg - 1,
            _ => {}
        }
    }

    (accumulator, is_loop)
}

fn change_op(instruction: &mut Instruction) {
    if instruction.op == "jmp" {instruction.op = String::from("nop");}
    else if instruction.op == "nop" {instruction.op = String::from("jmp");}
}

fn main() {
    let args: Vec<String> = env::args().collect();

	let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines = file_contents.lines();

    let mut instructions: Vec<Instruction> = lines.map(|l| parse_instruction_line(l)).collect();
    
    let (initial_acc, initial_is_loop) = run(&mut instructions);
    println!("{}, {}", initial_acc, initial_is_loop);

    let mut final_acc = 0;
    for i in 0 .. instructions.len() {
        {
            let instruction = &mut instructions[i];
            if instruction.op == "acc" { continue; }

            change_op(instruction);
        }
        let (acc, is_loop) = run(&mut instructions);
        if !is_loop {
            final_acc = acc;
            break;
        } else {
            let instruction = &mut instructions[i];
            change_op(instruction);
        }
    }
    println!("{}", final_acc);
}
