use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Inputs {
    rules: HashMap<u32, Vec<u32>>,
    manuals: Vec<Vec<u32>>,
}

impl Inputs {
    pub fn new() -> Self {
        return Inputs {
            rules: HashMap::new(),
            manuals: Vec::new(),
        };
    }
}

enum ParserState {
    Rules,
    Manuals,
}

fn read_rule(rules: &mut HashMap<u32, Vec<u32>>, line: String) {
    let parts: Vec<&str> = line.split("|").collect();
    let first = parts
        .get(0)
        .expect("Invalid rule")
        .parse()
        .expect("Not a number");
    let second = parts
        .get(1)
        .expect("Invalid rule")
        .parse()
        .expect("Not a number");
    rules.entry(first).or_default().push(second);
}

fn read_manuals(manuals: &mut Vec<Vec<u32>>, line: String) {
    let pages: Vec<u32> = line
        .split(",")
        .map(|x| x.parse().expect("Invalid number"))
        .collect();
    manuals.push(pages);
}

fn read_inputs() -> Inputs {
    let file = File::open("./input").expect("Input file missing");
    let mut inputs = Inputs::new();
    let mut state = ParserState::Rules;
    for line in io::BufReader::new(file).lines().flatten() {
        if line == "" {
            state = ParserState::Manuals;
            continue;
        }
        match state {
            ParserState::Rules => read_rule(&mut inputs.rules, line),
            ParserState::Manuals => read_manuals(&mut inputs.manuals, line),
        }
    }
    return inputs;
}

fn is_sorted(manual: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (idx, page) in manual.iter().enumerate() {
        if let Some(after_pages) = rules.get(page) {
            for prev_idx in 0..idx {
                if after_pages.contains(&manual[prev_idx]) {
                    return false;
                }
            }
        }
    }
    return true;
}

fn part1(inputs: &Inputs) {
    let mut middle_sum = 0;
    for manual in inputs.manuals.iter() {
        if is_sorted(&manual, &inputs.rules) {
            middle_sum += manual[manual.len() / 2];
        }
    }
    println!("Total sum of valid middle pages: {}", middle_sum);
}

fn part2(inputs: &Inputs) {
    let mut middle_sum = 0;
    for manual in inputs.manuals.iter() {
        if !is_sorted(&manual, &inputs.rules) {
            let mut sorted_manual = manual.to_vec();
            sorted_manual.sort_by(|x, y| {
                if let Some(pages) = inputs.rules.get(x) {
                    if pages.contains(y) {
                        return Ordering::Less;
                    }
                } else if let Some(pages) = inputs.rules.get(y) {
                    if pages.contains(x) {
                        return Ordering::Greater;
                    }
                }
                return Ordering::Equal;
            });
            middle_sum += sorted_manual[sorted_manual.len() / 2]
        }
    }
    println!(
        "Total sum of ordered, previously invalid, middle pages: {}",
        middle_sum
    );
}

fn main() {
    let inputs = read_inputs();
    println!(
        "Read {} rulesets for {} manuals",
        inputs.rules.len(),
        inputs.manuals.len()
    );
    part1(&inputs);
    part2(&inputs);
}
