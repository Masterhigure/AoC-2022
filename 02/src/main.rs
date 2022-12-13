use std::fs;

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(str::to_string)
        .collect::<Vec<_>>()
}

fn task_one(i: &[String]) -> i32 {
    let mut score = 0;
    for g in i.iter().map(String::as_bytes) {
        if g.is_empty() {
            continue;
        }
        let o = g[0] as i32 - 64;
        let m = g[2] as i32 - 87;
        let r = (m - o + 3 + 1) % 3;
        let t = m + 3 * r;
        score += t;
    }
    score
}

fn task_two(i: &[String]) -> i32 {
    let mut score = 0;
    for g in i.iter().map(String::as_bytes) {
        if g.is_empty() {
            continue;
        }
        let o = g[0] as i32 - 64;
        let r = g[2] as i32 - 88;
        let m = (r + o + 4) % 3 + 1;
        let t = m + 3 * r;
        score += t;
    }
    score
}

fn main() {
    let input = read_input();
    println!("Task 1: {}", task_one(&input[..]));
    println!("Task 2: {}", task_two(&input[..]));
}
