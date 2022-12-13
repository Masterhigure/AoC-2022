use std::fs;

fn read_input() -> Vec<u8> {
    fs::read("input.txt").unwrap()
}

fn task(i: &[u8], size: usize) -> usize {
    'outer: for (j, w) in i.windows(size).enumerate() {
        for u in 0..size {
            for v in (u + 1)..size {
                if w[u] == w[v] {
                    continue 'outer;
                }
            }
        }
        return j + size;
    }
    0
}

fn main() {
    let input = read_input();
    println!("Task 1: {}", task(&input[..], 4));
    println!("Task 2: {}", task(&input[..], 14));
}
