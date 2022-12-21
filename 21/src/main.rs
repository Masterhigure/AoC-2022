#![allow(unused)]
use std::{collections::HashMap, fs};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Monkey {
    Number(f64),
    Operation(String, Operation, String),
}

impl Monkey {
    fn value(&self, monkeys: &HashMap<String, Monkey>) -> f64 {
        match self {
            Self::Number(i) => *i,
            Self::Operation(n1, o, n2) => {
                let v1 = monkeys.get(n1).unwrap().value(monkeys);
                let v2 = monkeys.get(n2).unwrap().value(monkeys);
                match o {
                    Operation::Add => v1 + v2,
                    Operation::Sub => v1 - v2,
                    Operation::Mul => v1 * v2,
                    Operation::Div => v1 / v2,
                }
            }
        }
    }
}

type Input = HashMap<String, Monkey>;

pub fn read_input(filename: &str) -> Input {
    fs::read_to_string(filename)
        .unwrap()
        .split('\n')
        .map(|s| {
            let (name, o) = s.split_once(": ").unwrap();
            if let Ok(i) = o.parse() {
                (name.to_string(), Monkey::Number(i))
            } else {
                let mut o = o.split(' ');
                let n1 = o.next().unwrap();
                let op = match o.next().unwrap() {
                    "+" => Operation::Add,
                    "-" => Operation::Sub,
                    "*" => Operation::Mul,
                    "/" => Operation::Div,
                    _ => unreachable!(),
                };
                let n2 = o.next().unwrap();
                (
                    name.to_string(),
                    Monkey::Operation(n1.to_string(), op, n2.to_string()),
                )
            }
        })
        .collect::<HashMap<_, _>>()
}

pub fn task_one(i: Input) -> f64 {
    i.get("root").unwrap().value(&i)
}

pub fn task_two(mut i: Input) -> f64 {
    if let Monkey::Operation(m1, _, m2) = i.get("root").unwrap().clone() {
        i.insert(
            "root".to_string(),
            Monkey::Operation(m1, Operation::Sub, m2),
        );
    } else {
        unreachable!();
    }
    let mut step = 100.0;
    loop {
        let d1 = i.get("root").unwrap().value(&i);
        let &Monkey::Number(v) = i.get("humn").unwrap() else { unreachable!()};
        i.insert("humn".to_string(), Monkey::Number(v + step));
        let d2 = i.get("root").unwrap().value(&i);
        if d2.abs() < 0.01 {
            return v + step;
        } else if d1.abs() < 0.01 {
            return v;
        } else if d1 == d2 {
            if step < 100.0 {
                step *= 10.0;
            } else {
                panic!();
            }
            continue;
        }
        i.insert(
            "humn".to_string(),
            Monkey::Number(v + step * d1 / (d1 - d2)),
        );
        if step / (d1 - d2).abs() < 3.0 {
            step /= 3.0;
        }
    }
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
        assert_eq!(task_one(input.clone()), 152.0);
        assert_eq!(task_two(input), 301.0);
    }
}
