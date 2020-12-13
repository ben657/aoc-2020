use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Tile {
    seat: bool,
    occupied: bool
}

fn run(state: &Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, bool) {
    let mut result = Vec::new();
    for row in state {
        let mut new_row = Vec::new();
        for tile in row {
            new_row.push(tile.clone());
        }
        result.push(new_row);
    }

    let height = state.len() as isize;
    let width = state[0].len() as isize;

    let mut stable = true;

    for y in 0 .. height {
        for x in 0 .. width {
            let tile = &state[y as usize][x as usize];
            if !tile.seat { continue }
            
            let mut adjacent_occupied = 0;
            for y2 in y - 1 .. y + 1 {
                if y2 < 0 || y2 > height - 1 { continue }

                for x2 in x - 1 .. x + 1 {
                    if x2 < 0 || x2 > width - 1 { continue }
                    let tile = &state[y2 as usize][x2 as usize];
                    if tile.seat && tile.occupied { adjacent_occupied += 1}
                }
            }

            if !tile.occupied && adjacent_occupied == 0 {
                result[y as usize][x as usize].occupied = true;
                stable = false;
            } else if tile.occupied && adjacent_occupied >= 4 {
                result[y as usize][x as usize].occupied = false;
                stable = false;
            }
        }
    }

    (result, stable)
}

fn print_state(state: &Vec<Vec<Tile>>) {
    println!("---------------------");
    for row in state.iter() {
        for col in row.iter() {
            let c = if col.occupied { '#' } else if col.seat { 'L' } else { '.' };
            print!("{}", c);
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines = file_contents.lines();

    let mut map: Vec<Vec<Tile>> = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Tile {
                seat: c == 'L',
                occupied: false
            });
        }
        map.push(row);
    }

    let mut count = 0;
    loop {
        count += 1;
        let (new_map, stable) = run(&map);
        if stable { break }
        else { map = new_map }
        print_state(&map);
    }

    println!("Stable after {} iterations", count);
}
