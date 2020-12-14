use std::env;
use std::fs;
use regex::Regex;

mod ship;
use ship::*;

fn parse_instruction(line: &str) -> Instruction {
    let amount = line[1..].parse::<isize>().unwrap();
    let instruction_char = line[..1].chars().next().unwrap();
    match instruction_char {
        'N'|'E'|'S'|'W' => Instruction::Translate(Facing::from_char(&instruction_char), amount),
        'F' => Instruction::Forward(amount),
        'L'|'R' => Instruction::Turn(TurnDirection::from_char(&instruction_char), amount),
        _ => Instruction::None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines = file_contents.lines();

    let mut ship = Ship::new();

    let instructions: Vec<Instruction> = lines.map(|l| parse_instruction(l)).collect();
    for i in instructions.iter() {
        ship.run(i);
    }

    println!("{:?}", ship);
    println!("Distance {}", ship.x.abs() + ship.y.abs())    
}
