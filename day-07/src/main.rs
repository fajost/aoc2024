use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Equation {
    result: u64,
    values: Vec<u64>,
}

fn read_inputs() -> Vec<Equation> {
    let file = File::open("./input").expect("Input file missing");
    let mut equations = vec![];
    for line in io::BufReader::new(file).lines().flatten() {
        let split: Vec<&str> = line.split(":").collect();
        let result = split
            .get(0)
            .expect("Missing equation result")
            .parse()
            .expect("Invalid number");
        let values = split
            .get(1)
            .expect("Missing equation values")
            .trim()
            .split(" ")
            .map(|x| x.parse().expect("Invalid number"))
            .collect();
        equations.push(Equation { result, values })
    }
    return equations;
}

fn check_equation<'a, I>(total: u64, current: u64, mut iterator: I, combined_op: bool) -> bool
where
    I: Iterator<Item = &'a u64> + Clone,
{
    if current > total {
        return false;
    }
    let Some(next) = iterator.next() else {
        if total == current {
            return true;
        }
        return false;
    };
    if combined_op {
        let combined = format!("{}{}", current, next)
            .parse()
            .expect("Number too large");
        if check_equation(total, combined, iterator.clone(), combined_op) {
            return true;
        }
    }
    if check_equation(total, current * next, iterator.clone(), combined_op) {
        return true;
    }
    if check_equation(total, current + next, iterator, combined_op) {
        return true;
    }
    return false;
}

fn part1(equations: &Vec<Equation>) {
    let result: u64 = equations
        .iter()
        .filter(|e| {
            let mut iter = e.values.iter();
            let first = iter.next().expect("No values");
            check_equation(e.result, *first, iter, false)
        })
        .map(|e| e.result)
        .sum();
    println!(
        "Total test values of valid calibrations (2 ops): {}",
        result
    );
}

fn part2(equations: &Vec<Equation>) {
    let result: u64 = equations
        .iter()
        .filter(|e| {
            let mut iter = e.values.iter();
            let first = iter.next().expect("No values");
            check_equation(e.result, *first, iter, true)
        })
        .map(|e| e.result)
        .sum();
    println!(
        "Total test values of valid calibrations (3 ops): {}",
        result
    );
}

fn main() {
    let equations = read_inputs();
    println!("Found {} equations", equations.len());
    part1(&equations);
    part2(&equations);
}
