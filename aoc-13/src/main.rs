use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

	let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines: Vec<&str> = file_contents.lines().collect();
    
    let arrival_time = lines[0].parse::<isize>().unwrap();
    let busses: Vec<isize> = lines[1].split(',').filter(|b| *b != "x").map(|b| b.parse::<isize>().unwrap()).collect();
    
    let mut check_time = arrival_time;
    let result = loop {
        let mut result = -1;
        for bus in busses.iter() {
            if check_time % bus == 0 {
                let wait_time = check_time - arrival_time;
                result = bus * wait_time;
            }
        }
        if result >= 0 { break result; }
        check_time += 1;
    };

    println!("{}", result);

    let mut check_time = 0;
    let busses: Vec<isize> = lines[1].split(',').map(|b| if b == "x" {0} else {b.parse::<isize>().unwrap()}).collect();
    let mut jump = 1;
    for (i, bus) in busses.iter().enumerate() {
        if *bus == 0 {continue};
        while (check_time + i as isize) % bus != 0 {
            check_time += jump;
        } 

        jump *= bus;
    }

    println!("{}", check_time);
}
