use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq)]
struct Map<I>(Vec<Vec<I>>);

impl<I> Map<I> {
    fn iter(&self) -> MapIter<I> {
        MapIter {
            map: self,
            row: 0,
            col: 0,
        }
    }

    fn adjacents(&self, pos: &MapPosition<I>) -> Vec<MapPosition<I>> {
        let mut adjacents: Vec<MapPosition<I>> = vec![];
        for (delta_row, delta_col) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let Some(new_row) = pos.row.checked_add_signed(delta_row) else {
                continue;
            };
            let Some(new_col) = pos.col.checked_add_signed(delta_col) else {
                continue;
            };
            if new_row < self.0.len() && new_col < self.0[0].len() {
                adjacents.push(MapPosition {
                    value: &self.0[new_row][new_col],
                    row: new_row,
                    col: new_col,
                });
            }
        }
        return adjacents;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MapPosition<'a, I> {
    value: &'a I,
    row: usize,
    col: usize,
}

struct MapIter<'a, I> {
    map: &'a Map<I>,
    row: usize,
    col: usize,
}

impl<'a, I> Iterator for MapIter<'a, I> {
    type Item = MapPosition<'a, I>;

    fn next(&mut self) -> Option<Self::Item> {
        self.col += 1;
        if self.col >= self.map.0[self.row].len() {
            self.row += 1;
            self.col = 0;
        }
        if self.row >= self.map.0.len() {
            return None;
        }
        return Some(MapPosition {
            value: &self.map.0[self.row][self.col],
            row: self.row,
            col: self.col,
        });
    }
}

fn read_inputs() -> Map<u8> {
    let file = File::open("./input").expect("Input file missing");
    return Map(io::BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse().expect("Not a number"))
                .collect()
        })
        .collect());
}

fn part1(map: &Map<u8>) {
    let mut total = 0;
    for start in map.iter().filter(|x| *x.value == 0) {
        let mut queue: VecDeque<_> = [start].into();
        let mut reached: Vec<MapPosition<u8>> = vec![];
        while let Some(position) = queue.pop_front() {
            if *position.value == 9 && !reached.contains(&position) {
                reached.push(position);
            }
            for adjacent in map
                .adjacents(&position)
                .iter()
                .filter(|x| *x.value == *position.value + 1)
            {
                queue.push_back(*adjacent);
            }
        }
        total += reached.len();
    }
    println!("Total of ratings (part 1): {}", total);
}

fn part2(map: &Map<u8>) {
    let mut total = 0;
    for start in map.iter().filter(|x| *x.value == 0) {
        let mut queue: VecDeque<_> = [start].into();
        while let Some(position) = queue.pop_front() {
            if *position.value == 9 {
                total += 1;
            }
            for adjacent in map
                .adjacents(&position)
                .iter()
                .filter(|x| *x.value == *position.value + 1)
            {
                queue.push_back(*adjacent);
            }
        }
    }
    println!("Total of ratings (part 2): {}", total);
}

fn main() {
    let map = read_inputs();
    part1(&map);
    part2(&map);
}
