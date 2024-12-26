use std::fs;

#[derive(Debug, Clone, Copy)]
struct File {
    index: u64,
    size: u64,
}

#[derive(Debug)]
enum Partition {
    Free(u64, Vec<File>),
    Used(File),
}

fn part1() {
    let mut content: Vec<u64> = fs::read_to_string("./input")
        .expect("Missing input files")
        .chars()
        .filter_map(|c| c.to_string().parse().ok())
        .collect();
    println!("Content len: {}", content.len());
    let mut total: u64 = 0;
    let mut start: usize = 0;
    let mut end: usize = content.len() - 1;
    let mut idx: u64 = 0;
    loop {
        // println!("{} {} {} {} {:?}", total, start, end, idx, content);
        if start % 2 == 0 {
            // file
            if content[start] > 0 {
                content[start] -= 1;
                total += (start / 2) as u64 * idx;
                idx += 1;
            } else {
                start += 1;
            }
        } else {
            // free
            while content[end] == 0 && end > start {
                end -= 2;
            }
            if end <= start {
                break;
            }
            if content[start] > 0 {
                content[start] -= 1;
                content[end] -= 1;
                total += (end / 2) as u64 * idx;
                idx += 1;
            } else {
                start += 1;
            }
        }
    }
    println!("Calculate hash: {}", total);
}

fn part2() {
    let mut content: Vec<Partition> = fs::read_to_string("./input")
        .expect("Missing input files")
        .chars()
        .filter_map(|c| c.to_string().parse().ok())
        .enumerate()
        .map(|(idx, val)| {
            if idx % 2 == 0 {
                Partition::Used(File {
                    index: idx as u64 / 2,
                    size: val,
                })
            } else {
                Partition::Free(val, vec![])
            }
        })
        .collect();
    for file_idx in (0..content.len()).rev() {
        let Partition::Used(file) = content[file_idx] else {
            continue;
        };
        for free_idx in 0..file_idx {
            let Partition::Free(ref mut size, ref mut files) = content[free_idx] else {
                continue;
            };
            if *size >= file.size {
                files.push(file);
                *size -= file.size;
                content[file_idx] = Partition::Free(file.size, vec![]);
                break;
            }
        }
    }
    let mut hash: u64 = 0;
    let mut idx: u64 = 0;
    for part in content.iter() {
        match part {
            Partition::Used(file) => {
                for _ in 0..file.size {
                    hash += idx * file.index;
                    idx += 1;
                }
            }
            Partition::Free(size, files) => {
                for file in files.iter() {
                    for _ in 0..file.size {
                        hash += idx * file.index;
                        idx += 1;
                    }
                }
                idx += size;
            }
        }
    }
    println!("Calculated hash with partition: {}", hash);
}

fn main() {
    part1();
    part2();
}
