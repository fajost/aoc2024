use auto_ops::impl_op_ex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

type Map = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    fn new(row: i32, col: i32) -> Self {
        Coordinate { row, col }
    }
}

impl_op_ex!(+ |a: &Coordinate, b: &Coordinate| -> Coordinate {
    Coordinate::new(a.row + b.row, a.col + b.col)
});
impl_op_ex!(-|a: &Coordinate, b: &Coordinate| -> Coordinate {
    Coordinate::new(a.row - b.row, a.col - b.col)
});
impl_op_ex!(*|a: &Coordinate, b: &i32| -> Coordinate { Coordinate::new(a.row * b, a.col * b) });

struct AntinodeResult {
    max_rows: i32,
    max_cols: i32,
    nodes: Vec<Coordinate>,
}

impl AntinodeResult {
    fn new(map: &Map) -> Self {
        AntinodeResult {
            max_rows: map.len().try_into().expect("Map to large"),
            max_cols: map[0].len().try_into().expect("Map to large"),
            nodes: vec![],
        }
    }

    fn valid(&self, node: &Coordinate) -> bool {
        if 0 <= node.row && node.row < self.max_rows && 0 <= node.col && node.col < self.max_cols {
            return true;
        }
        return false;
    }

    fn push(&mut self, node: Coordinate) {
        if self.valid(&node) && !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }
}

fn read_inputs() -> Map {
    let file = File::open("./input").expect("Input file missing");
    return io::BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect();
}

fn map_to_coordinates(map: &Map) -> HashMap<char, Vec<Coordinate>> {
    let mut coords = HashMap::<char, Vec<Coordinate>>::new();
    for (row, row_values) in map.iter().enumerate() {
        for (col, value) in row_values.iter().enumerate() {
            if *value == '.' {
                continue;
            }
            coords.entry(*value).or_default().push(Coordinate {
                row: row.try_into().expect("Map too large"),
                col: col.try_into().expect("Map too large"),
            });
        }
    }
    return coords;
}

fn part1(map: &Map) {
    let coords = map_to_coordinates(map);
    let mut antinodes = AntinodeResult::new(map);
    for (_, locations) in coords.iter() {
        for (idx, loc1) in locations.iter().enumerate() {
            for loc2 in locations.iter().skip(idx + 1) {
                let delta = loc1 - loc2;
                for node in vec![loc1 + delta, loc2 - delta] {
                    antinodes.push(node);
                }
            }
        }
    }
    println!("Total number of antinodes: {}", antinodes.nodes.len());
}

fn part2(map: &Map) {
    let coords = map_to_coordinates(map);
    let mut antinodes = AntinodeResult::new(map);
    for (_, locations) in coords.iter() {
        for (idx, loc1) in locations.iter().enumerate() {
            for loc2 in locations.iter().skip(idx + 1) {
                let delta = loc1 - loc2;
                let mut mult = 0;
                while antinodes.valid(&(loc1 + delta * mult))
                    || antinodes.valid(&(loc1 - delta * mult))
                {
                    antinodes.push(loc1 + delta * mult);
                    antinodes.push(loc1 - delta * mult);
                    mult += 1;
                }
            }
        }
    }
    println!(
        "Total number of antinodes (with harmonics): {}",
        antinodes.nodes.len()
    );
}

fn main() {
    let map = read_inputs();
    println!("Read map of size {}, {}", map.len(), map[0].len());
    part1(&map);
    part2(&map);
}
