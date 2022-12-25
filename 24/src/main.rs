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

type Input = (HashSet<(Point, Point)>, i32, i32);

fn read_input(filename: &str) -> Input {
    let blizzards = fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .enumerate()
        .flat_map(|(i, s)| {
            s.bytes().enumerate().filter_map(move |(j, u)| match u {
                b'>' => Some((
                    Point {
                        x: j as i32,
                        y: i as i32,
                    },
                    Point { x: 1, y: 0 },
                )),
                b'v' => Some((
                    Point {
                        x: j as i32,
                        y: i as i32,
                    },
                    Point { x: 0, y: 1 },
                )),
                b'<' => Some((
                    Point {
                        x: j as i32,
                        y: i as i32,
                    },
                    Point { x: -1, y: 0 },
                )),
                b'^' => Some((
                    Point {
                        x: j as i32,
                        y: i as i32,
                    },
                    Point { x: 0, y: -1 },
                )),
                b'#' => Some((
                    Point {
                        x: j as i32,
                        y: i as i32,
                    },
                    Point { x: 0, y: 0 },
                )),
                _ => None,
            })
        })
        .collect();
    let map = fs::read_to_string(filename).unwrap();
    let map = map.split('\n').collect::<Vec<_>>();
    let maxx = map[0].len() - 1;
    let maxy = map.len() - 1;
    (blizzards, maxx as i32, maxy as i32)
}

fn visualize(blizz: &HashSet<(Point, Point)>, pos: &HashSet<Point>, maxx: i32, maxy: i32) {
    let mut s = "".to_string();
    for y in 0..=maxy {
        for x in 0..=maxx {
            let q = Point { x, y };
            if blizz.iter().any(|&(p, d)| p == q && (d.x != 0 || d.y != 0)) {
                s.push('B');
            } else if blizz.iter().any(|&(p, d)| p == q && d.x == 0 && d.y == 0) {
                s.push('#');
            } else if pos.contains(&q) {
                s.push('E');
            } else {
                s.push(' ');
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

const DIRS: [Point; 5] = [
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: 0, y: -1 },
    Point { x: -1, y: 0 },
];

pub fn task_one(mut i: Input) -> i32 {
    let mut blizz = i.0;
    let mut pos = HashSet::new();
    pos.insert(Point { x: 1, y: 0 });
    for k in 1.. {
        let mut new_blizz = HashSet::new();
        for (p, d) in blizz.drain() {
            let mut new_p = p + d;
            if new_p.x == 0 && d.x == -1 {
                new_p.x = i.1 - 1;
            }
            if new_p.x == i.1 && d.x == 1 {
                new_p.x = 1;
            }
            if new_p.y == 0 && d.y == -1 {
                new_p.y = i.2 - 1;
            }
            if new_p.y == i.2 && d.y == 1 {
                new_p.y = 1;
            }
            new_blizz.insert((new_p, d));
        }
        blizz = new_blizz;
        let mut new_pos = HashSet::new();
        for p in pos {
            for d in DIRS {
                let new_p = p + d;
                if !blizz.iter().any(|&(p, _)| p == new_p)
                    && new_p.x >= 0
                    && new_p.x <= i.1
                    && new_p.y >= 0
                    && new_p.y <= i.2
                {
                    new_pos.insert(p + d);
                }
            }
        }
        pos = new_pos;
        if pos.contains(&Point { x: i.1 - 1, y: i.2 }) {
            return k;
        }
    }
    0
}

pub fn task_two(i: Input) -> i32 {
    let mut phase = 1;
    let mut blizz = i.0;
    let mut pos = HashSet::new();
    pos.insert(Point { x: 1, y: 0 });
    for k in 1.. {
        let mut new_blizz = HashSet::new();
        for (p, d) in blizz.drain() {
            let mut new_p = p + d;
            if new_p.x == 0 && d.x == -1 {
                new_p.x = i.1 - 1;
            }
            if new_p.x == i.1 && d.x == 1 {
                new_p.x = 1;
            }
            if new_p.y == 0 && d.y == -1 {
                new_p.y = i.2 - 1;
            }
            if new_p.y == i.2 && d.y == 1 {
                new_p.y = 1;
            }
            new_blizz.insert((new_p, d));
        }
        blizz = new_blizz;
        let mut new_pos = HashSet::new();
        for p in pos {
            for d in DIRS {
                let new_p = p + d;
                if !blizz.iter().any(|&(p, _)| p == new_p)
                    && new_p.x >= 0
                    && new_p.x <= i.1
                    && new_p.y >= 0
                    && new_p.y <= i.2
                {
                    new_pos.insert(p + d);
                }
            }
        }
        pos = new_pos;
        if pos.contains(&Point { x: i.1 - 1, y: i.2 }) {
            if phase == 1 {
                pos.clear();
                pos.insert(Point { x: i.1 - 1, y: i.2 });
                phase = 2;
            }
            if phase == 3 {
                return k;
            }
        }
        if phase == 2 && pos.contains(&Point { x: 1, y: 0 }) {
            pos.clear();
            pos.insert(Point { x: 1, y: 0 });
            phase = 3;
        }
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
    fn test_one() {
        let input = read_input("example.txt");
        assert_eq!(task_one(input), 18);
    }

    #[test]
    fn test_main() {
        let input = read_input("example.txt");
        assert_eq!(task_two(input), 54);
    }
}
