use std::{collections::HashSet, fs};

fn read_input() -> Vec<(u8, i32)> {
    fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .filter(|&s| !s.is_empty())
        .map(|s| {
            let (dir, dist) = s.split_once(' ').unwrap();
            (dir.bytes().next().unwrap(), dist.parse().unwrap())
        })
        .collect()
}

fn task<const N: usize>(i: &[(u8, i32)]) -> usize {
    let mut visited = HashSet::new();
    let mut rope = [(0i32, 0i32); N];
    for &(dir, dist) in i {
        for _ in 0..dist {
            match dir {
                b'R' => {
                    rope[0].0 += 1;
                }
                b'L' => {
                    rope[0].0 -= 1;
                }
                b'U' => {
                    rope[0].1 += 1;
                }
                b'D' => {
                    rope[0].1 -= 1;
                }
                _ => unreachable!(),
            }
            for k in 1..N {
                if (rope[k - 1].0 - rope[k].0).abs() > 1 || (rope[k - 1].1 - rope[k].1).abs() > 1 {
                    rope[k].0 += (rope[k - 1].0 - rope[k].0).signum();
                    rope[k].1 += (rope[k - 1].1 - rope[k].1).signum();
                }
            }
            visited.insert(rope[N - 1]);
        }
    }
    visited.len()
}

fn main() {
    let input = read_input();
    println!("Task 1: {}", task::<2>(&input[..]));
    println!("Task 2: {}", task::<10>(&input[..]));
}
