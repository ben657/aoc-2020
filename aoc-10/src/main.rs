use std::env;
use std::fs;
use std::collections::HashMap;

fn abs(n: isize) -> isize {
    if n < 0 {
        n * -1
    } else {
        n
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines = file_contents.lines();

    let mut adapters: Vec<usize> = lines.map(|l| l.parse::<usize>().unwrap()).collect();
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);
    println!("{:?}", adapters);

    // let mut differences = HashMap::new();
    // for i in 0 .. adapters.len() - 1 {
    //     let diff = adapters[i + 1] - adapters[i];
    //     *differences.entry(diff).or_insert(0) += 1;
    // }

    // let key_1 = 1;
    // let key_3 = 3;

    // let diffs_1 = differences.get(&key_1).unwrap();
    // let diffs_3 = differences.get(&key_3).unwrap();
    // println!("1 diffs: {}, 3 diffs: {}, result: {}", diffs_1, diffs_3, diffs_1 * diffs_3);

    let mut combinations: HashMap<usize, i64> = HashMap::new();
    combinations.insert(0, 1);

    for j in &adapters[1..] {
        let a = if *j < 1 { 0 } else { *combinations.entry(j - 1).or_insert(0) };
        let b = if *j < 2 { 0 } else { *combinations.entry(j - 2).or_insert(0) };
        let c = if *j < 3 { 0 } else { *combinations.entry(j - 3).or_insert(0) };
        combinations.insert(*j, a + b + c);
    }

    println!("Combinations: {}", combinations[&adapters[adapters.len() - 1]]);
}
