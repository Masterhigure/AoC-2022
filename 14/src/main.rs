#![allow(unused)]
use std::{collections::HashSet, fs};

fn read_input() -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    let rocks = fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|s| {
            s.split(" -> ")
                .map(|t| {
                    let (first, second) = t.split_once(',').unwrap();
                    (
                        first.parse::<usize>().unwrap(),
                        second.parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for v in rocks {
        for (&s, &e) in v.iter().zip(v[1..].iter()) {
            if s.0 == e.0 {
                for y in s.1.min(e.1)..=s.1.max(e.1) {
                    result.insert((s.0, y));
                }
            } else {
                for x in s.0.min(e.0)..=s.0.max(e.0) {
                    result.insert((x, s.1));
                }
            }
        }
    }
    result
}

fn task_one(mut i: HashSet<(usize, usize)>) -> i32 {
    let mut grains = 0;
    let ymax = i.iter().map(|&(_, y)| y).max().unwrap();
    'outer: loop {
        let mut grain = (500, 0);
        loop {
            if !i.contains(&(grain.0, grain.1 + 1)) {
                grain.1 += 1;
            } else if !i.contains(&(grain.0 - 1, grain.1 + 1)) {
                grain.0 -= 1;
                grain.1 += 1;
            } else if !i.contains(&(grain.0 + 1, grain.1 + 1)) {
                grain.0 += 1;
                grain.1 += 1;
            } else {
                i.insert(grain);
                grains += 1;
                continue 'outer;
            }
            if grain.1 > ymax {
                break 'outer;
            }
        }
    }
    grains
}

fn task_two(mut i: HashSet<(usize, usize)>) -> i32 {
    let mut grains = 0;
    let ymax = i.iter().map(|&(_, y)| y).max().unwrap();
    'outer: loop {
        let mut grain = (500, 0);
        loop {
            if !i.contains(&(grain.0, grain.1 + 1)) {
                grain.1 += 1;
            } else if !i.contains(&(grain.0 - 1, grain.1 + 1)) {
                grain.0 -= 1;
                grain.1 += 1;
            } else if !i.contains(&(grain.0 + 1, grain.1 + 1)) {
                grain.0 += 1;
                grain.1 += 1;
            } else {
                i.insert(grain);
                grains += 1;
                if grain == (500, 0) {
                    break 'outer;
                }
                continue 'outer;
            }
            if grain.1 > ymax {
                i.insert(grain);
                grains += 1;
                continue 'outer;
            }
        }
    }
    grains
}

fn main() {
    let input = read_input();
    /*
    // Printout of map
    let xmin = input.iter().map(|&(x, _)| x).min().unwrap();
    let xmax = input.iter().map(|&(x, _)| x).max().unwrap();
    let ymax = input.iter().map(|&(_, y)| y).max().unwrap();
    let xrange = xmax - xmin + 2;
    let yrange = ymax + 1;
    let s = (1..(xrange * yrange))
        .map(|u| {
            if input.contains(&(u % xrange + xmin - 1, u / xrange)) {
                '#'
            } else if u % xrange == 0 {
                '\n'
            } else {
                ' '
            }
        })
        .collect::<String>();
    println!("{}", s);
    */
    println!("Task 1: {}", task_one(input.clone()));
    println!("Task 2: {}", task_two(input));
}
