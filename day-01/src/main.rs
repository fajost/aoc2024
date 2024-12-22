use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct Input {
    list1: Vec<u32>,
    list2: Vec<u32>,
}

fn read_inputs() -> Input {
    let file = File::open("./input").expect("Input file should be present");
    let sep = Regex::new(r"\s+").expect("Invalid regex");
    let mut list1 = Vec::<u32>::new();
    let mut list2 = Vec::<u32>::new();
    for line in io::BufReader::new(file).lines().flatten() {
        let parts = sep.split(&line).collect::<Vec<&str>>();
        list1.push(parts[0].parse().unwrap());
        list2.push(parts[1].parse().unwrap());
    }
    return Input { list1, list2 };
}

fn part1() {
    let mut input = read_inputs();
    input.list1.sort();
    input.list2.sort();
    let result: u32 = input
        .list1
        .into_iter()
        .zip(input.list2.into_iter())
        .map(|(x, y)| x.abs_diff(y))
        .sum();
    println!("The difference of the two lists is {}", result);
}

fn part2() {
    let input = read_inputs();
    let mut frequency = HashMap::<u32, u32>::new();
    for val in input.list2 {
        *frequency.entry(val).or_default() += 1;
    }
    let result: u32 = input
        .list1
        .into_iter()
        .map(|x| x * frequency.get(&x).unwrap_or(&0))
        .sum();
    println!("The similarity of the two lists is {}", result);
}

fn main() {
    part1();
    part2();
}
