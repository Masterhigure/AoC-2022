#![allow(unused)]
use std::{fs, ops::Add};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

const NORTH: Point = Point { x: 0, y: -1 };
const SOUTH: Point = Point { x: 0, y: 1 };
const EAST: Point = Point { x: 1, y: 0 };
const WEST: Point = Point { x: -1, y: 0 };

const DIRECTIONS: [Point; 4] = [NORTH, SOUTH, EAST, WEST];

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    coords: Point,
    height: u8,
}

fn read_input() -> Vec<Vec<u8>> {
    fs::read("input.txt")
        .unwrap()
        .iter()
        .map(|&b| b ^ 0b01100000)
        .collect::<Vec<_>>()
        .split(|&b| b == b'\n' ^ 0b01100000)
        .map(|s| s.to_vec())
        .filter(|v| !v.is_empty())
        .collect()
}

fn task_one(i: Vec<Vec<u8>>) -> i32 {
    let max_x = i.len() as i32 - 1;
    let max_y = i[0].len() as i32 - 1;
    let (start_x, v) = i.iter().enumerate().find(|&v| v.1.contains(&51)).unwrap();
    let start_y = v.iter().position(|&u| u == 51).unwrap();
    let mut to_visit = vec![(
        0,
        Point {
            x: start_x as i32,
            y: start_y as i32,
        },
    )];
    let mut visited = Vec::new();
    while let Some(p) = to_visit.pop() {
        let mut h = i[p.1.x as usize][p.1.y as usize];
        if h == 37 {
            return p.0;
        }
        if h == 51 {
            h = 1
        }
        visited.push(p);
        for d in DIRECTIONS {
            let np = p.1 + d;
            if visited.iter().any(|&(_, q)| q == np) {
                continue;
            }
            if np.x < 0 || np.x > max_x || np.y < 0 || np.y > max_y {
                continue;
            }
            let nh = i[np.x as usize][np.y as usize];
            if nh > h + 1 && !(h >= 25 && nh == 37) {
                continue;
            }
            if let Some((mut dist, q)) = to_visit.iter().find(|&&(_, q)| q == np) {
                if dist > p.0 + 1 {
                    dist = p.0 + 1;
                    to_visit.sort_by_key(|&(dist, _)| -dist);
                }
            } else {
                to_visit.push((p.0 + 1, np));
                to_visit.sort_by_key(|&(dist, _)| -dist);
            }
        }
    }
    0
}

fn task_two(i: Vec<Vec<u8>>) -> i32 {
    let max_x = i.len() as i32 - 1;
    let max_y = i[0].len() as i32 - 1;
    let mut to_visit = Vec::new();
    for x in 0..=max_x {
        for y in 0..=max_y {
            if i[x as usize][y as usize] == 1 || i[x as usize][y as usize] == 51 {
                to_visit.push((0, Point { x, y }));
            }
        }
    }
    let mut visited = Vec::new();
    while let Some(p) = to_visit.pop() {
        let mut h = i[p.1.x as usize][p.1.y as usize];
        if h == 37 {
            return p.0;
        }
        if h == 51 {
            h = 1
        }
        visited.push(p);
        for d in DIRECTIONS {
            let np = p.1 + d;
            if visited.iter().any(|&(_, q)| q == np) {
                continue;
            }
            if np.x < 0 || np.x > max_x || np.y < 0 || np.y > max_y {
                continue;
            }
            let nh = i[np.x as usize][np.y as usize];
            if nh > h + 1 && !(h >= 25 && nh == 37) {
                continue;
            }
            if let Some((mut dist, q)) = to_visit.iter().find(|&&(_, q)| q == np) {
                if dist > p.0 + 1 {
                    dist = p.0 + 1;
                    to_visit.sort_by_key(|&(dist, _)| -dist);
                }
            } else {
                to_visit.push((p.0 + 1, np));
                to_visit.sort_by_key(|&(dist, _)| -dist);
            }
        }
    }
    0
}

fn main() {
    let input = read_input();
    println!("Task 1: {}", task_one(input.clone()));
    println!("Task 2: {}", task_two(input));
}
