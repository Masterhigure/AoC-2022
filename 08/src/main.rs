use std::fs;

fn read_input() -> Vec<Vec<u8>> {
    fs::read("input.txt")
        .unwrap()
        .split(|&u| u == 0x0a)
        .filter(|&s| !s.is_empty())
        .map(|v| v.to_vec())
        .collect()
}

fn task_one(i: &[Vec<u8>]) -> i32 {
    let mut counter = 0;
    for k in 0..(i.len()) {
        for l in 0..(i[0].len()) {
            let up = (0..k).all(|k1| i[k1][l] < i[k][l]);
            let down = ((k + 1)..(i.len())).all(|k2| i[k2][l] < i[k][l]);
            let left = (0..l).all(|l1| i[k][l1] < i[k][l]);
            let right = ((l + 1)..(i[0].len())).all(|l2| i[k][l2] < i[k][l]);
            if up || down || left || right {
                counter += 1;
            }
        }
    }
    counter
}

fn task_two(i: &[Vec<u8>]) -> usize {
    let mut max_view = 0;
    for k in 1..(i.len() - 1) {
        for l in 1..(i[0].len() - 1) {
            let up = (0..k)
                .rev()
                .position(|k1| i[k1][l] >= i[k][l])
                .unwrap_or(k - 1)
                + 1;
            let down = ((k + 1)..(i.len()))
                .position(|k2| i[k2][l] >= i[k][l])
                .unwrap_or(i.len() - k - 2)
                + 1;
            let left = (0..l)
                .rev()
                .position(|l1| i[k][l1] >= i[k][l])
                .unwrap_or(l - 1)
                + 1;
            let right = ((l + 1)..(i[0].len()))
                .position(|l2| i[k][l2] >= i[k][l])
                .unwrap_or(i[0].len() - l - 2)
                + 1;
            max_view = max_view.max(left * right * up * down);
        }
    }
    max_view
}

fn main() {
    let input = read_input();
    println!("Task 1: {}", task_one(&input[..]));
    println!("Task 2: {}", task_two(&input[..]));
}
