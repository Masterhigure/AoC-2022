use std::fs;

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(str::to_string)
        .collect::<Vec<_>>()
}

fn task_one(_i: &[String]) -> i32 {
    0
}

fn task_two(_i: &[String]) -> i32 {
    0
}

fn main() {
    let input = read_input();
    println!("Task 1: {}", task_one(&input[..]));
    println!("Task 2: {}", task_two(&input[..]));
}
