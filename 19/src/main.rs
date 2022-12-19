#![allow(unused)]
#![feature(int_roundings)]
use std::{
    fs,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct Cost {
    ore: i32,
    clay: i32,
    obsidian: i32,
}

impl Add for Cost {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Cost {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
        }
    }
}

impl Sub for Cost {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Cost {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
        }
    }
}

impl Div for Cost {
    type Output = Option<i32>;

    fn div(self, bots: Self) -> Self::Output {
        if (self.ore > 0 && bots.ore == 0)
            || (self.clay > 0 && bots.clay == 0)
            || (self.obsidian > 0 && bots.obsidian == 0)
        {
            return None;
        }
        let o = if self.ore <= 0 {
            0
        } else {
            self.ore.div_ceil(bots.ore)
        };
        let c = if self.clay <= 0 {
            0
        } else {
            self.clay.div_ceil(bots.clay)
        };
        let g = if self.obsidian <= 0 {
            0
        } else {
            self.obsidian.div_ceil(bots.obsidian)
        };
        Some(o.max(c).max(g))
    }
}

impl Mul<i32> for Cost {
    type Output = Self;

    fn mul(self, k: i32) -> Self::Output {
        Cost {
            ore: self.ore * k,
            clay: self.clay * k,
            obsidian: self.obsidian * k,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct Blueprint {
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

type Input = Vec<Blueprint>;

pub fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|l| {
            let raw = l
                .split(' ')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>();
            Blueprint {
                ore: Cost {
                    ore: raw[0],
                    ..Default::default()
                },
                clay: Cost {
                    ore: raw[1],
                    ..Default::default()
                },
                obsidian: Cost {
                    ore: raw[2],
                    clay: raw[3],
                    ..Default::default()
                },
                geode: Cost {
                    ore: raw[4],
                    obsidian: raw[5],
                    ..Default::default()
                },
            }
        })
        .collect::<Vec<_>>()
}

fn score(b: Blueprint, time: i32, mut resources: Cost, mut robots: Cost) -> i32 {
    if time >= 24 {
        return 0;
    }
    let mut best = 0;
    if let Some(t) = (b.ore - resources) / robots {
        let mut new_robots = robots;
        new_robots.ore += 1;
        let s = score(
            b,
            time + t + 1,
            resources + robots * (t + 1) - b.ore,
            new_robots,
        );
        if s > best {
            best = s;
        }
    }
    if let Some(t) = (b.clay - resources) / robots {
        let mut new_robots = robots;
        new_robots.clay += 1;
        let s = score(
            b,
            time + t + 1,
            resources + robots * (t + 1) - b.clay,
            new_robots,
        );
        if s > best {
            best = s;
        }
    }
    if let Some(t) = (b.obsidian - resources) / robots {
        let mut new_robots = robots;
        new_robots.obsidian += 1;
        let s = score(
            b,
            time + t + 1,
            resources + robots * (t + 1) - b.obsidian,
            new_robots,
        );
        if s > best {
            best = s;
        }
    }
    if let Some(t) = (b.geode - resources) / robots {
        let s = score(
            b,
            time + t + 1,
            resources + robots * (t + 1) - b.geode,
            robots,
        ) + (23 - time - t);
        if s > best {
            best = s;
        }
    }
    best
}

fn score_with_prune(
    b: Blueprint,
    time: i32,
    mut resources: Cost,
    mut robots: Cost,
    score_so_far: i32,
    best_ever: i32,
) -> i32 {
    if time >= 32 {
        return 0;
    }

    let time_to_clay = if robots.clay != 0 {
        0
    } else {
        let mut t = 0;
        let mut o = b.clay.ore - resources.ore;
        let mut r = robots.ore;
        while o > 0 {
            o -= r;
            r += 1;
            t += 1;
        }
        t + 1
    };
    let time_to_obsidian = if robots.obsidian != 0 {
        0
    } else {
        let mut t = 0;
        let mut o = b.obsidian.clay - resources.clay;
        let mut r = robots.clay;
        while o > 0 {
            o -= r;
            r += 1;
            t += 1;
        }
        t + 1
    } + time_to_clay;
    let mut possible_geodes = 0;
    let mut r = robots.obsidian;
    let mut o = resources.obsidian;
    for t in (time + time_to_obsidian)..32 {
        o += r;
        if o - r >= b.geode.obsidian {
            possible_geodes += 31 - t;
        } else {
            r += 1;
        }
    }

    if score_so_far + possible_geodes < best_ever {
        return 0;
    }

    let mut best = 0;
    let mut be = best_ever;
    if let Some(t) = (b.ore - resources) / robots {
        let mut new_robots = robots;
        new_robots.ore += 1;
        let s = score_with_prune(
            b,
            time + t + 1,
            resources + robots * (t + 1) - b.ore,
            new_robots,
            score_so_far,
            be,
        );
        if s > best {
            best = s;
        }
        if s + score_so_far > be {
            be = s + score_so_far;
        }
    }
    if let Some(t) = (b.clay - resources) / robots {
        let mut new_robots = robots;
        new_robots.clay += 1;
        let s = score_with_prune(
            b,
            time + t + 1,
            resources + robots * (t + 1) - b.clay,
            new_robots,
            score_so_far,
            be,
        );
        if s > best {
            best = s;
        }
        if s + score_so_far > be {
            be = s + score_so_far;
        }
    }
    if let Some(t) = (b.obsidian - resources) / robots {
        let mut new_robots = robots;
        new_robots.obsidian += 1;
        let s = score_with_prune(
            b,
            time + t + 1,
            resources + robots * (t + 1) - b.obsidian,
            new_robots,
            score_so_far,
            be,
        );
        if s > best {
            best = s;
        }
        if s + score_so_far > be {
            be = s + score_so_far;
        }
    }
    if let Some(t) = (b.geode - resources) / robots {
        let s = score_with_prune(
            b,
            time + t + 1,
            resources + robots * (t + 1) - b.geode,
            robots,
            score_so_far + (31 - time - t),
            be,
        ) + (31 - time - t);
        if s > best {
            best = s;
        }
        if s + score_so_far > be {
            be = s + score_so_far;
        }
    }
    best
}

pub fn task_one(mut i: Input) -> i32 {
    let mut points = 0;
    for (k, b) in i.drain(..).enumerate() {
        points += (k as i32 + 1)
            * score(
                b,
                0,
                Cost::default(),
                Cost {
                    ore: 1,
                    ..Default::default()
                },
            );
    }
    points
}

pub fn task_two(mut i: Input) -> i32 {
    let mut points = 1;
    for (k, b) in i.drain(..(3.min(i.len()))).enumerate() {
        points *= score_with_prune(
            b,
            0,
            Cost::default(),
            Cost {
                ore: 1,
                ..Default::default()
            },
            0,
            0,
        );
    }
    points
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
        assert_eq!(task_one(input.clone()), 33);
        assert_eq!(task_two(input), 56 * 62);
    }
}
