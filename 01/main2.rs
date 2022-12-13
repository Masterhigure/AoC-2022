use std::fs;

fn main() {
    let input1 = fs::read_to_string("input/1.txt").unwrap();
    let mut elves = input1
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .filter_map(|s| s.parse::<i32>().ok())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();
    elves.sort_by_key(|&i| -i);
    let result1 = elves[0];
    println!("Task 1 result: {}", result1);

    let result2 = elves[0] + elves[1] + elves[2];
    println!("Task 2 result: {}", result2);
}
