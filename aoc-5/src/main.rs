use std::env;
use std::fs;

struct BoardingPass {
    row_code: String,
    col_code: String
}

struct Bound {
    from: isize,
    size: usize
}

fn calc_pass_id(pass: &BoardingPass) -> isize {
    let mut row_bounds = Bound {
        from: 0,
        size: 128
    };

    for c in pass.row_code.chars() {
        row_bounds.size /= 2;
        if c == 'B' {
            row_bounds.from += row_bounds.size as isize;
        }
    }

    let mut col_bounds = Bound {
        from: 0,
        size: 8
    };
    
    for c in pass.col_code.chars() {
        col_bounds.size /= 2;
        if c == 'R' {
            col_bounds.from += col_bounds.size as isize;
        }
    }

    println!("{} - {}", row_bounds.from, col_bounds.from);

    row_bounds.from * 8 + col_bounds.from
}

fn main() {
    let args: Vec<String> = env::args().collect();

	let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines = file_contents.lines();
    
    let passes = lines.map(|l| {
        BoardingPass {
            row_code: String::from(&l[..7]),
            col_code: String::from(&l[7..])
        }
    });

    let mut results: Vec<isize> = passes.map(|p| calc_pass_id(&p)).collect();
    results.sort();
    let mut seat = 0;
    for i in 1 .. results.len() - 1 {
        if results[i - 1] + 1 != results[i] {
            seat = results[i - 1] + 1;
        }
    }

    println!("Your seat is {}", seat);
}
