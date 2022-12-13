#![allow(unused)]
use std::{cmp::Ordering, fs, str::FromStr};

#[derive(Eq, PartialEq, Clone, Debug)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.clone(), other.clone()) {
            (Self::Int(s), Self::Int(t)) => s.cmp(&t),
            (Self::Int(s), o) => Packet::List(vec![Packet::Int(s)]).cmp(&o),
            (s, Self::Int(t)) => s.cmp(&Packet::List(vec![Packet::Int(t)])),
            (Self::List(v), Self::List(w)) => {
                for (k, l) in v.iter().zip(w.iter()) {
                    let c = k.cmp(l);
                    if c != Ordering::Equal {
                        return c;
                    }
                }
                v.len().cmp(&w.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            return Ok(Packet::Int(s.parse().unwrap()));
        }
        let mut result = Packet::List(Vec::new());
        let mut count = 0;
        let mut old_pos = 1;
        for (pos, c) in s.chars().enumerate() {
            if c == '[' {
                count += 1;
            }
            if c == ']' {
                count -= 1;
            }
            if c == ',' && count == 1 {
                if let Packet::List(ref mut v) = result {
                    v.push(s[old_pos..pos].parse().unwrap());
                }
                old_pos = pos + 1;
            }
        }
        if let Packet::List(ref mut v) = result {
            if s != "[]" {
                v.push(s[old_pos..(s.chars().count() - 1)].parse().unwrap());
            }
        }
        Ok(result)
    }
}

fn read_input() -> Vec<(Packet, Packet)> {
    fs::read_to_string("input.txt")
        .unwrap()
        .split("\n\n")
        .map(|p| {
            let (one, two) = p.split_once('\n').unwrap();
            (one.parse().unwrap(), two.parse().unwrap())
        })
        .collect::<Vec<_>>()
}

fn task_one(i: &[(Packet, Packet)]) -> usize {
    i.iter()
        .enumerate()
        .filter(|(_, (p, q))| p <= q)
        .map(|(u, _)| u + 1)
        .sum()
}

fn task_two(i: &[(Packet, Packet)]) -> usize {
    let mut packets = vec!["[[6]]".parse().unwrap(), "[[2]]".parse().unwrap()];
    for (p, q) in i {
        packets.push(p.clone());
        packets.push(q.clone());
    }
    packets.sort();
    let pos1 = packets.binary_search(&("[[6]]".parse().unwrap())).unwrap();
    let pos2 = packets.binary_search(&("[[2]]".parse().unwrap())).unwrap();
    (pos1 + 1) * (pos2 + 1)
}

fn main() {
    let input = read_input();
    println!("Task 1: {}", task_one(&input[..]));
    println!("Task 2: {}", task_two(&input[..]));
}
