use std::fs::File;
use std::io::{self, BufRead};

fn read_inputs() -> Vec<Vec<i32>> {
    let file = File::open("./input").expect("Input file should be present");
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in io::BufReader::new(file).lines().flatten() {
        let parts = line.split(" ");
        reports.push(parts.map(|x| x.parse().unwrap()).collect());
    }
    return reports;
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut is_increasing: Option<bool> = None;
    for idx in 1..report.len() {
        let delta = report[idx] - report[idx - 1];
        if delta > 0 {
            if is_increasing == Some(false) || delta > 3 {
                return false;
            }
            is_increasing = Some(true);
        } else if delta < 0 {
            if is_increasing == Some(true) || delta < -3 {
                return false;
            }
            is_increasing = Some(false);
        } else {
            return false;
        }
    }
    return true;
}

fn check_vec(deltas: &Vec<i32>, lower: i32, upper: i32) -> bool {
    let gaps: Vec<usize> = deltas
        .iter()
        .enumerate()
        .filter(|(_, x)| **x < lower || **x > upper)
        .map(|(idx, _)| idx)
        .collect();
    match gaps.len() {
        0 => return true,
        1 => {
            let idx = gaps[0];
            if idx == 0 || idx == deltas.len() - 1 {
                // first or last element can always be removed
                return true;
            }
            if (lower..=upper).contains(&(deltas[idx - 1] + deltas[idx]))
                || (lower..=upper).contains(&(deltas[idx] + deltas[idx + 1]))
            {
                return true;
            }
            return false;
        }
        2 => {
            let idx1 = gaps[0];
            let idx2 = gaps[1];
            if idx1.abs_diff(idx2) != 1 {
                // must be two adjacent numbers
                return false;
            }
            if (lower..=upper).contains(&(deltas[idx1] + deltas[idx2])) {
                return true;
            }
            return false;
        }
        _ => return false,
    };
}

fn is_safe_dampener(report: &Vec<i32>) -> bool {
    let deltas: Vec<i32> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(cur, next)| next - cur)
        .collect();
    return check_vec(&deltas, -3, -1) || check_vec(&deltas, 1, 3);
}

fn part1(reports: &Vec<Vec<i32>>) {
    let result = reports.iter().map(|x| is_safe(&x)).filter(|x| *x).count();
    println!("Number of safe reports: {}", result);
}

fn part2(reports: &Vec<Vec<i32>>) {
    let result = reports
        .iter()
        .map(|x| is_safe_dampener(&x))
        .filter(|x| *x)
        .count();
    println!("Number of safe reports with dampener: {}", result);
}

fn main() {
    let reports = read_inputs();
    part1(&reports);
    part2(&reports);
}
