use regex::Regex;
use std::fs;

fn part1() {
    let expr_re = Regex::new(r"mul\((?<num1>\d\d?\d?),(?<num2>\d\d?\d?)\)")
        .expect("Invalid expression regex");
    let content = fs::read_to_string("./input").expect("Missing input files");
    let result: u32 = expr_re
        .captures_iter(content.as_str())
        .map(|found| found["num1"].parse::<u32>().unwrap() * found["num2"].parse::<u32>().unwrap())
        .sum();
    println!("The total product is {}", result);
}

fn part2() {
    let expr_re = Regex::new(
        r"(?<mul>mul\((?<num1>\d\d?\d?),(?<num2>\d\d?\d?)\))|(?<do>do\(\))|(?<dont>don't\(\))",
    )
    .expect("Invalid expression regex");
    let content = fs::read_to_string("./input").expect("Missing input files");
    let mut active = true;
    let mut result: u32 = 0;
    for expr in expr_re.captures_iter(content.as_str()) {
        match expr.get(0).unwrap().as_str() {
            "do()" => active = true,
            "don't()" => active = false,
            _ => {
                if active {
                    result +=
                        expr["num1"].parse::<u32>().unwrap() * expr["num2"].parse::<u32>().unwrap()
                }
            }
        }
    }
    println!("The total product with conditionals is {}", result);
}

fn main() {
    part1();
    part2();
}
