#![allow(unused)]
use std::fs;

type Input = Vec<(usize, i64)>;

pub fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| s.parse().unwrap())
        .enumerate()
        .collect::<Vec<_>>()
}

pub fn task_one(mut i: Input) -> i64 {
    for j in 0..i.len() {
        let mut k = i.iter().position(|&(o, _)| o == j).unwrap();
        let (_, v) = i.remove(k);
        k = (k as i64 + v).rem_euclid(i.len() as i64) as usize;
        i.insert(k, (0, v));
    }
    let zero = i.iter().position(|&(_, v)| v == 0).unwrap();
    i[(zero + 1000) % i.len()].1 + i[(zero + 2000) % i.len()].1 + i[(zero + 3000) % i.len()].1
}

pub fn task_two(mut i: Input) -> i64 {
    for x in 0..10 {
        for j in 0..i.len() {
            let mut k = i.iter().position(|&(o, _)| o == j).unwrap();
            let (o, mut v) = i.remove(k);
            v *= 811589153;
            k = (k as i64 + v).rem_euclid(i.len() as i64) as usize;
            i.insert(k, (o, v / 811589153));
        }
    }
    let zero = i.iter().position(|&(_, v)| v == 0).unwrap();
    (i[(zero + 1000) % i.len()].1 + i[(zero + 2000) % i.len()].1 + i[(zero + 3000) % i.len()].1)
        * 811589153
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
        assert_eq!(task_one(input), 3);
    }

    #[test]
    fn test_two() {
        let input = read_input("example.txt");
        assert_eq!(task_two(input), 1623178306);
    }
}
