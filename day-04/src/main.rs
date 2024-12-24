use std::fs::File;
use std::io::{self, BufRead};

fn read_inputs() -> Vec<Vec<char>> {
    let file = File::open("./input").expect("Input file missing");
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    for line in io::BufReader::new(file).lines().flatten() {
        puzzle.push(line.chars().collect());
    }
    return puzzle;
}

fn get_value(
    puzzle: &Vec<Vec<char>>,
    row_idx: usize,
    col_idx: usize,
    row_dir: isize,
    col_dir: isize,
) -> Option<&char> {
    let Some(row_new) = row_idx.checked_add_signed(row_dir) else {
        return None;
    };
    let Some(col_new) = col_idx.checked_add_signed(col_dir) else {
        return None;
    };
    if let Some(row) = puzzle.get(row_new) {
        return row.get(col_new);
    } else {
        return None;
    };
}

fn check_direction(
    puzzle: &Vec<Vec<char>>,
    row_idx: usize,
    col_idx: usize,
    row_dir: isize,
    col_dir: isize,
) -> bool {
    let letters = vec!['X', 'M', 'A', 'S'];
    for (idx, letter) in letters.iter().enumerate() {
        if Some(letter)
            != get_value(
                puzzle,
                row_idx,
                col_idx,
                idx as isize * row_dir,
                idx as isize * col_dir,
            )
        {
            return false;
        }
    }
    return true;
}

fn part1(puzzle: &Vec<Vec<char>>) {
    let mut count = 0;
    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for row_idx in 0..puzzle.len() {
        for col_idx in 0..puzzle[row_idx].len() {
            if puzzle[row_idx][col_idx] == 'X' {
                for (row_dir, col_dir) in directions.iter() {
                    if check_direction(puzzle, row_idx, col_idx, *row_dir, *col_dir) {
                        count += 1
                    }
                }
            }
        }
    }
    println!("Number of XMAS: {}", count);
}

fn part2(puzzle: &Vec<Vec<char>>) {
    let mut count = 0;
    for row_idx in 0..puzzle.len() {
        for col_idx in 0..puzzle[row_idx].len() {
            if puzzle[row_idx][col_idx] == 'A' {
                let tl = get_value(puzzle, row_idx, col_idx, -1, -1);
                let tr = get_value(puzzle, row_idx, col_idx, -1, 1);
                let bl = get_value(puzzle, row_idx, col_idx, 1, -1);
                let br = get_value(puzzle, row_idx, col_idx, 1, 1);
                match (tl, br) {
                    (Some('M'), Some('S')) => (),
                    (Some('S'), Some('M')) => (),
                    _ => continue,
                }
                match (tr, bl) {
                    (Some('M'), Some('S')) => (),
                    (Some('S'), Some('M')) => (),
                    _ => continue,
                }
                count += 1;
            }
        }
    }
    println!("Number of X-MAS: {}", count);
}

fn main() {
    let puzzle = read_inputs();
    part1(&puzzle);
    part2(&puzzle);
}
