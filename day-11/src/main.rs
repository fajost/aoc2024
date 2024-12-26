use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug)]
enum Tree<T> {
    Item(T),
    Split(Box<Self>, Box<Self>),
}

impl<T> From<&[T]> for Tree<T>
where
    T: Copy,
{
    fn from(value: &[T]) -> Self {
        match value.len() {
            0 => panic!("Cannot create from empty vec"),
            1 => Self::Item(value[0]),
            _ => {
                let (left, right) = value.split_at(value.len() / 2);
                Self::Split(Box::new(Self::from(left)), Box::new(Self::from(right)))
            }
        }
    }
}

struct TreeIterMut<'a, T>(VecDeque<&'a mut Tree<T>>);

impl<'a, T> Tree<T> {
    fn iter_mut(&'a mut self) -> TreeIterMut<'a, T> {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        TreeIterMut(queue)
    }
}

impl<'a, T> Iterator for TreeIterMut<'a, T> {
    type Item = &'a mut Tree<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.0.pop_front() {
            match node {
                Tree::Split(left, right) => {
                    self.0.push_front(right);
                    self.0.push_front(left);
                }
                x => return Some(x),
            }
        }
        return None;
    }
}

fn read_inputs() -> Tree<u64> {
    return Tree::from(
        fs::read_to_string("./input")
            .expect("Missing input files")
            .trim()
            .split(" ")
            .map(|c| c.parse().expect("Not a number"))
            .collect::<Vec<u64>>()
            .as_slice(),
    );
}

fn part1() {
    let mut tree = read_inputs();
    for _ in 0..25 {
        for node in tree.iter_mut() {
            let Tree::Item(val) = node else {
                continue;
            };
            let sval = val.to_string();
            if *val == 0 {
                *val += 1;
            } else if sval.len() % 2 == 0 {
                let (left, right) = sval.split_at(sval.len() / 2);
                *node = Tree::Split(
                    Box::new(Tree::Item(left.parse().unwrap())),
                    Box::new(Tree::Item(right.parse().unwrap())),
                )
            } else {
                *val *= 2024
            }
        }
    }
    println!("Total of stones: {}", tree.iter_mut().count());
}

fn part2() {
    let mut values = HashMap::<u64, u64>::from_iter(
        fs::read_to_string("./input")
            .expect("Missing input files")
            .trim()
            .split(" ")
            .map(|c| (c.parse().expect("Not a number"), 1))
            .collect::<Vec<(u64, u64)>>(),
    );
    for _ in 0..75 {
        let mut new_values = HashMap::<u64, u64>::new();
        for (val, count) in values.iter() {
            let string = val.to_string();
            if *val == 0 {
                *new_values.entry(1).or_default() += count;
            } else if string.len() % 2 == 0 {
                let (left, right) = string.split_at(string.len() / 2);
                *new_values.entry(left.parse().unwrap()).or_default() += count;
                *new_values.entry(right.parse().unwrap()).or_default() += count;
            } else {
                *new_values.entry(*val * 2024).or_default() += count;
            }
        }
        values = new_values;
    }
    println!(
        "Total of stones (75 iterations): {}",
        values.values().sum::<u64>()
    );
}

fn main() {
    part1();
    part2();
}
