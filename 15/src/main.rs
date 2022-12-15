#![allow(unused)]
use std::fs;

fn read_input() -> Vec<(i32, i32, i32, i32)> {
    fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|s| {
            let n = s
                .split(['=', ',', ':'])
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>();
            (n[0], n[1], n[2], n[3])
        })
        .collect::<Vec<_>>()
}

fn dist(i: (i32, i32, i32, i32)) -> i32 {
    (i.0 - i.2).abs() + (i.1 - i.3).abs()
}

// Dumb bruteforce
fn task_one(i: &[(i32, i32, i32, i32)]) -> usize {
    let row = 2_000_000;
    let minx = i.iter().map(|&q| (q.0 - dist(q))).min().unwrap();
    let maxx = i.iter().map(|&q| (q.0 + dist(q))).max().unwrap();
    (minx..maxx)
        .filter(|&pos| {
            i.iter().any(|&(x, y, xb, yb)| {
                !(xb == pos && yb == row)
                    && (x - pos).abs() + (y - row).abs() <= dist((x, y, xb, yb))
            })
        })
        .count()
}

// Row-by-row bruteforce, but each row a little smarter than in 1
fn task_two(i: &[(i32, i32, i32, i32)]) -> usize {
    let size = 4_000_000;
    for row in 0..=size {
        let minx = 0;
        let maxx = size;
        let mut pos = minx;
        'row: while pos <= maxx {
            for &(x, y, xb, yb) in i {
                let d = dist((x, y, xb, yb));
                if (x - pos).abs() + (y - row).abs() <= d {
                    pos = x + d - (y - row).abs() + 1;
                    continue 'row;
                }
            }
            return (pos as usize) * 4_000_000 + (row as usize);
        }
    }
    0
}

fn main() {
    let input = read_input();
    println!("Task 1: {}", task_one(&input[..]));
    println!("Task 2: {}", task_two(&input[..]));
}
