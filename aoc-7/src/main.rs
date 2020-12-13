use std::env;
use std::fs;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use regex::Regex;

const BAG_COL_REGEX: &str = r"(.*) bags contain .*";
const CONTAINING_REGEX: &str = r"(?:(\d+) (.+?) bags?(?:,|\.) ?)";
const CONTAINING_NONE_REGEX: &str = r".*no other bags.";

#[derive(Debug)]
struct Bag {
    color: String,
    contained_by: HashSet<String>,
    can_contain: HashMap<String, usize>
}

fn find_bag_color(line: &str) -> String {
    let bag_col_regex = Regex::new(BAG_COL_REGEX).unwrap();
    let captures = bag_col_regex.captures(line).unwrap();
    String::from(captures.get(1).unwrap().as_str())
}

fn find_bag_empty_required(line: &str) -> bool {
    let bag_empty_regex = Regex::new(CONTAINING_NONE_REGEX).unwrap();
    bag_empty_regex.is_match(line)
}

fn find_bag_can_contain(line: &str) -> HashMap<String, usize> {
    let bag_containing_regex = Regex::new(CONTAINING_REGEX).unwrap();
    let captures = bag_containing_regex.captures_iter(line);

    let mut result = HashMap::new();
    for cap in captures {
        result.insert(String::from(&cap[2]), cap[1].parse::<usize>().unwrap());
    }
    result
}

fn parse_bag_line(line: &str) -> RefCell<Bag>{
    let bag_required_empty = find_bag_empty_required(line);

    let bag = Bag {
        color: find_bag_color(line),
        contained_by: HashSet::new(),
        can_contain: if bag_required_empty {HashMap::new()} else {find_bag_can_contain(line)}
    };
    RefCell::new(bag)
}

fn find_parent_colors(target: &String, bags: &Vec<RefCell<Bag>>) -> HashSet<String> {
    let target_parents = bags.iter().filter(|bag| bag.borrow().can_contain.contains_key(target)).map(|bag| bag.borrow().color.clone());
    let mut parents_list: HashSet<String> = HashSet::from_iter(target_parents);

    let mut rest = HashSet::new();
    for p in parents_list.iter() {
        let parents = find_parent_colors(&p, bags);
        for p2 in parents {
            rest.insert(p2);
        }
    }

    for p in rest {
        parents_list.insert(p);
    }

    parents_list
}

fn count_children(target: &String, bag_map: &HashMap<String, &RefCell<Bag>>) -> usize {
    let bag = bag_map.get(target).unwrap().borrow();
    let mut children = 0;
    
    for (color, amount) in bag.can_contain.iter() {
        children += amount + (amount * count_children(color, bag_map));
    }
    children
}

fn main() {
    let args: Vec<String> = env::args().collect();

	let filename = &args[1];

	let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let lines = file_contents.lines();
    
    let bags: Vec<RefCell<Bag>> = lines.map(|l| parse_bag_line(l)).collect();
    let mut bags_by_color = HashMap::new();
    for bag in bags.iter() {
        bags_by_color.insert(bag.borrow().color.clone(), bag);
    }
    for (color, bag) in bags_by_color.iter() {
        let base_bag = bag.borrow();
        for (c, _) in base_bag.can_contain.iter() {
            let contained_bag_cell = bags_by_color.get(c).unwrap();
            if contained_bag_cell.borrow().color == base_bag.color { break; }
            let mut contained_bag = contained_bag_cell.borrow_mut();
            contained_bag.contained_by.insert(color.clone());
        }
    }

    let target_color = String::from("shiny gold");

    let amount = count_children(&target_color, &bags_by_color);

    println!("{:?}", amount);
}
