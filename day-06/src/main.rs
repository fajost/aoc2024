use std::fs::File;
use std::io::{self, BufRead};

#[derive(Eq, PartialEq, Clone)]
enum MapElement {
    Obstacle,
    Empty,
    Visited(Vec<Direction>),
}

impl MapElement {
    fn from_char(c: char) -> MapElement {
        match c {
            '#' => MapElement::Obstacle,
            '.' => MapElement::Empty,
            '^' => MapElement::Visited(vec![Direction::Up]),
            _ => panic!("Invalid map input"),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        };
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn step(&self, direction: &Direction) -> Option<Position> {
        let (offset_row, offset_col): (isize, isize) = match direction {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };
        if let Some(new_row) = self.row.checked_add_signed(offset_row) {
            if let Some(new_col) = self.col.checked_add_signed(offset_col) {
                return Some(Position {
                    row: new_row,
                    col: new_col,
                });
            }
        }
        return None;
    }
}

#[derive(Clone)]
struct Situation {
    map: Vec<Vec<MapElement>>,
    position: Position,
    direction: Direction,
}

#[derive(PartialEq, Eq)]
enum PathStatus {
    Exited(usize),
    Loops,
}

fn read_inputs() -> Situation {
    let file = File::open("./input").expect("Input file missing");
    let mut map = Vec::new();
    let mut guard = None;
    for (row, line) in io::BufReader::new(file).lines().flatten().enumerate() {
        if let Some(col) = line.chars().position(|x| x == '^') {
            guard = Some(Position { row, col })
        }
        map.push(line.chars().map(MapElement::from_char).collect());
    }
    return Situation {
        map,
        position: guard.expect("No guard found"),
        direction: Direction::Up,
    };
}

fn follow_path(situation: &mut Situation, check_loops: bool) -> PathStatus {
    let mut obstacle_options: Vec<Position> = vec![];
    let initial_pos = situation.position;
    while let Some(new_pos) = situation.position.step(&situation.direction) {
        if let Some(map_row) = situation.map.get(new_pos.row) {
            if let Some(field) = map_row.get(new_pos.col) {
                match field {
                    MapElement::Empty => {
                        if check_loops {
                            let mut new_situation = situation.clone();
                            new_situation.map[new_pos.row][new_pos.col] = MapElement::Obstacle;
                            new_situation.direction.turn();
                            if follow_path(&mut new_situation, false) == PathStatus::Loops {
                                if !obstacle_options.contains(&new_pos) && new_pos != initial_pos {
                                    obstacle_options.push(new_pos);
                                }
                            }
                        }

                        situation.map[new_pos.row][new_pos.col] =
                            MapElement::Visited(vec![situation.direction]);
                        situation.position = new_pos;
                    }
                    MapElement::Visited(dirs) => {
                        if dirs.contains(&situation.direction) {
                            return PathStatus::Loops;
                        }
                        let mut new_dirs = dirs.clone();
                        new_dirs.push(situation.direction);
                        situation.map[new_pos.row][new_pos.col] = MapElement::Visited(new_dirs);
                        situation.position = new_pos;
                    }
                    MapElement::Obstacle => situation.direction.turn(),
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    return PathStatus::Exited(obstacle_options.len());
}

fn part1and2() {
    let mut situation = read_inputs();
    let result = follow_path(&mut situation, true);
    let count: usize = situation
        .map
        .iter()
        .map(|row| {
            row.iter()
                .filter(|x| match x {
                    MapElement::Visited(_) => true,
                    _ => false,
                })
                .count()
        })
        .sum();
    println!("Number of visited fields: {}", count);
    if let PathStatus::Exited(options) = result {
        println!("Number of obstacle options: {}", options);
    }
}

fn main() {
    part1and2();
}
