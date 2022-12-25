#![allow(unused)]
use std::fs;

type Input = Vec<i64>;

pub fn read_input(filename: &str) -> Input {
    fs::read(filename)
        .unwrap()
        .split(|&u| u == b'\n')
        .map(|s| {
            s.iter()
                .rev()
                .enumerate()
                .map(|(i, &u)| {
                    let p = 5i64.pow(i as u32);
                    p * match u {
                        b'0' => 0,
                        b'1' => 1,
                        b'2' => 2,
                        b'-' => -1,
                        b'=' => -2,
                        _ => unreachable!(),
                    }
                })
                .sum()
        })
        .collect::<Vec<_>>()
}

pub fn task_one(i: Input) -> String {
    let mut s = i.iter().sum::<i64>();
    let mut result = "".to_string();
    while s != 0 {
        let t = (s + 2) % 5 - 2;
        s = (s - t) / 5;
        result.push(match t {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        });
    }
    result.chars().rev().collect()
}

pub fn task_two(i: Input) -> i64 {
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
        assert_eq!(task_one(input), "2=-1=0".to_string());
    }

    #[test]
    fn test_two() {
        let input = read_input("example.txt");
        assert_eq!(task_two(input), 0);
    }
}
