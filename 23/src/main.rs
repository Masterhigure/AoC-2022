#![allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{Add, Sub},
};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

type Input = HashSet<Point>;

fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .enumerate()
        .flat_map(|(i, s)| {
            s.bytes().enumerate().filter_map(move |(j, u)| {
                if u == b'#' {
                    Some(Point {
                        x: j as i32,
                        y: i as i32,
                    })
                } else {
                    None
                }
            })
        })
        .collect()
}

fn visualise(i: &Input) {
    let minx = i.iter().map(|&p| p.x).min().unwrap();
    let maxx = i.iter().map(|&p| p.x).max().unwrap();
    let miny = i.iter().map(|&p| p.y).min().unwrap();
    let maxy = i.iter().map(|&p| p.y).max().unwrap();
    let mut chars = Vec::new();
    for k in miny..=maxy {
        let mut row = Vec::new();
        for l in minx..=maxx {
            if i.contains(&Point { x: l, y: k }) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        row.push('\n');
        chars.push(row);
    }
    println!("{}", chars.iter().flatten().collect::<String>());
}

const LOOKAROUND: [[Point; 3]; 4] = [
    [
        Point { x: 1, y: -1 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: -1 },
    ],
    [
        Point { x: 1, y: 1 },
        Point { x: 0, y: 1 },
        Point { x: -1, y: 1 },
    ],
    [
        Point { x: -1, y: 1 },
        Point { x: -1, y: 0 },
        Point { x: -1, y: -1 },
    ],
    [
        Point { x: 1, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 1, y: -1 },
    ],
];

fn task_one(mut i: Input) -> i32 {
    for k in 0..10 {
        let mut plans = HashMap::new();
        'elf: for &e in &i {
            if LOOKAROUND.iter().flatten().all(|&p| !i.contains(&(p + e))) {
                plans.insert(e, e);
                continue 'elf;
            }
            for l in 0..4 {
                if LOOKAROUND[(k + l) % 4]
                    .iter()
                    .all(|&p| !i.contains(&(p + e)))
                {
                    plans.insert(e, e + LOOKAROUND[(k + l) % 4][1]);
                    continue 'elf;
                }
            }
            plans.insert(e, e);
        }
        i.clear();
        for (&e, &p) in &plans {
            if let Some(&q) = plans.get(&(p + p - e)) {
                if q == p {
                    i.insert(e);
                    continue;
                }
            }
            i.insert(p);
        }
    }
    let minx = i.iter().map(|&p| p.x).min().unwrap();
    let maxx = i.iter().map(|&p| p.x).max().unwrap();
    let miny = i.iter().map(|&p| p.y).min().unwrap();
    let maxy = i.iter().map(|&p| p.y).max().unwrap();
    (maxx - minx + 1) * (maxy - miny + 1) - i.len() as i32
}

pub fn task_two(mut i: Input) -> i32 {
    for k in 0.. {
        let mut plans = HashMap::new();
        'elf: for &e in &i {
            if LOOKAROUND.iter().flatten().all(|&p| !i.contains(&(p + e))) {
                plans.insert(e, e);
                continue 'elf;
            }
            for l in 0..4 {
                if LOOKAROUND[(k + l) % 4]
                    .iter()
                    .all(|&p| !i.contains(&(p + e)))
                {
                    plans.insert(e, e + LOOKAROUND[(k + l) % 4][1]);
                    continue 'elf;
                }
            }
            plans.insert(e, e);
        }
        let mut j = HashSet::new();
        for (&e, &p) in &plans {
            if let Some(&q) = plans.get(&(p + p - e)) {
                if q == p {
                    j.insert(e);
                    continue;
                }
            }
            j.insert(p);
        }
        if j == i {
            return k as i32 + 1;
        }
        i = j;
    }
    0
}

fn main() {
    let input = read_input("input.txt");
    println!("Task 1: {}", task_one(input.clone()));
    println!("Task 2: {}", task_two(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {
        let input = read_input("example.txt");
        assert_eq!(task_one(input.clone()), 110);
        assert_eq!(task_two(input), 20);
    }
}
