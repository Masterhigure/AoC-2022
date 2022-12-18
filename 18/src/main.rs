#![allow(unused)]
use std::{collections::HashSet, fs};

type Input = HashSet<(i32, i32, i32)>;

pub fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| {
            let c = s.split(',').map(|u| u.parse().unwrap()).collect::<Vec<_>>();
            (c[0], c[1], c[2])
        })
        .collect::<HashSet<_>>()
}

pub fn task_one(i: Input) -> i32 {
    // The set sides contains _all_ sides of all cubes of the droplet (indexed by
    // direction of face and the position of the cube that's to the
    // left of / above / behind that face. And count counts
    // the number of sides that have been added twice, which are all the
    // shared sides. Subtract to get answer.

    let mut count = 0;
    let mut sides: HashSet<(i32, (i32, i32, i32))> = HashSet::new();
    for &c in &i {
        if !sides.insert((0, c)) {
            count += 1;
        }
        let mut c0 = c;
        c0.0 += 1;
        if !sides.insert((0, c0)) {
            count += 1;
        }
        if !sides.insert((1, c)) {
            count += 1;
        }
        let mut c1 = c;
        c1.1 += 1;
        if !sides.insert((1, c1)) {
            count += 1;
        }
        if !sides.insert((2, c)) {
            count += 1;
        }
        let mut c2 = c;
        c2.2 += 1;
        if !sides.insert((2, c2)) {
            count += 1;
        }
    }
    6 * i.len() as i32 - 2 * count
}

pub fn task_two(i: Input) -> i32 {
    //Make a "droplet" that consists of a rectangular box surrounding the
    //real droplet. Calculate the surface of that box using part 1, subtract the
    //external rectangular faces.

    let maxx = i.iter().map(|&(x, _, _)| x).max().unwrap();
    let minx = i.iter().map(|&(x, _, _)| x).min().unwrap();
    let maxy = i.iter().map(|&(_, x, _)| x).max().unwrap();
    let miny = i.iter().map(|&(_, x, _)| x).min().unwrap();
    let maxz = i.iter().map(|&(_, _, x)| x).max().unwrap();
    let minz = i.iter().map(|&(_, _, x)| x).min().unwrap();
    let mut neighs = HashSet::new();
    let mut outside = HashSet::new();
    neighs.insert((maxx + 1, maxy + 1, maxz + 1));
    while !neighs.is_empty() {
        let mut new_neighs = HashSet::new();
        for (x, y, z) in neighs.drain() {
            outside.insert((x, y, z));
            if x <= maxx && !outside.contains(&(x + 1, y, z)) && !i.contains(&(x + 1, y, z)) {
                new_neighs.insert((x + 1, y, z));
            }
            if x >= minx && !outside.contains(&(x - 1, y, z)) && !i.contains(&(x - 1, y, z)) {
                new_neighs.insert((x - 1, y, z));
            }
            if y <= maxy && !outside.contains(&(x, y + 1, z)) && !i.contains(&(x, y + 1, z)) {
                new_neighs.insert((x, y + 1, z));
            }
            if y >= miny && !outside.contains(&(x, y - 1, z)) && !i.contains(&(x, y - 1, z)) {
                new_neighs.insert((x, y - 1, z));
            }
            if z <= maxz && !outside.contains(&(x, y, z + 1)) && !i.contains(&(x, y, z + 1)) {
                new_neighs.insert((x, y, z + 1));
            }
            if z >= minz && !outside.contains(&(x, y, z - 1)) && !i.contains(&(x, y, z - 1)) {
                new_neighs.insert((x, y, z - 1));
            }
        }
        neighs = new_neighs;
    }
    let side_1 = (maxx - minx + 3) * (maxy - miny + 3);
    let side_2 = (maxx - minx + 3) * (maxz - minz + 3);
    let side_3 = (maxy - miny + 3) * (maxz - minz + 3);
    let o = task_one(outside);
    o - 2 * (side_1 + side_2 + side_3)
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
    fn test_advanced() {
        let mut i = HashSet::new();
        for x in 1..=3 {
            for y in 1..=3 {
                for z in 1..=3 {
                    if x == 2 && y == 2 && z == 2 {
                        continue;
                    }
                    i.insert((x, y, z));
                }
            }
        }
        assert_eq!(task_one(i.clone()), 6 * 9 + 6);
        assert_eq!(task_two(i), 6 * 9);
    }

    #[test]
    fn test_basic() {
        let mut i = HashSet::new();
        i.insert((1, 1, 1));
        assert_eq!(task_two(i), 6);
    }

    #[test]
    fn test_main() {
        let input = read_input("example.txt");
        assert_eq!(task_one(input.clone()), 64);
        assert_eq!(task_two(input), 58);
    }
}
