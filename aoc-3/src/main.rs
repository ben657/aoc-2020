use std::env;
use std::fs;

struct Slope {
    x: usize,
    y: usize
}

fn parse_slope(line: &str) -> Slope {
    let parts: Vec<&str> = line.split(' ').collect();

    Slope {
        x: parts[0].parse::<usize>().expect(""),
        y: parts[1].parse::<usize>().expect("")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let filename2 = &args[2];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines = file_contents.lines();

    let slopes_content = fs::read_to_string(filename2).expect("Error reading file 2");
    let slopes_lines = slopes_content.lines();
    let slopes = slopes_lines.map(|l| parse_slope(l));
    
    let mut map: Vec<Vec<char>> = Vec::new();
    for (line_num, line) in lines.enumerate() {
        map.push(Vec::new());
        for c in line.chars() {
            map[line_num].push(c);
        }
    }

    let seg_width = map[0].len();
    let seg_height = map.len();

    let mut results = Vec::new();

    for slope in slopes {
        let mut x_pos = 0;
        let mut y_pos = 0;
        let mut obstacle_count: i64 = 0;
        while y_pos < seg_height - slope.y {
            x_pos += slope.x;
            // Wrap round if we go off the edge
            if x_pos >= seg_width { x_pos %= seg_width }

            y_pos += slope.y;

            let tile = map[y_pos][x_pos];
            if tile == '#' {
                obstacle_count += 1;
            }
        }

        results.push(obstacle_count);
        println!("Hit {} obstacles", obstacle_count);
    }

    let mut result = results[0];
    for i in 1 .. results.len() {
        result *= results[i];
    }
    println!("Result {}", result);
}
