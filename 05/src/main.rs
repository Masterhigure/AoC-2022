use std::fs;

fn read_input() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let input = fs::read_to_string("input.txt").unwrap();
    let (start, moves) = input.split_once("\n\n").unwrap();
    let mut pile_desc = start.lines().collect::<Vec<_>>();
    let num_piles = (pile_desc.pop().unwrap().len() + 1) / 4;
    let mut piles = Vec::new();
    piles.resize(num_piles, Vec::new());

    for row in pile_desc {
        let row = row.chars().collect::<Vec<_>>();
        for i in 0..num_piles {
            if row[i * 4 + 1] != ' ' {
                piles[i].push(row[i * 4 + 1]);
            }
        }
    }
    piles.iter_mut().for_each(|r| r.reverse());

    let moves = moves
        .lines()
        .map(|r| {
            let words = r.split_whitespace().collect::<Vec<_>>();
            (
                words[1].parse().unwrap(),
                words[3].parse().unwrap(),
                words[5].parse().unwrap(),
            )
        })
        .collect();
    (piles, moves)
}

fn task_one(mut piles: Vec<Vec<char>>, moves: &[(usize, usize, usize)]) -> String {
    for &(n, f, t) in moves {
        for _ in 0..n {
            let c = piles[f - 1].pop().unwrap();
            piles[t - 1].push(c);
        }
    }
    piles.iter_mut().map(|r| r.pop().unwrap()).collect()
}

fn task_two(mut piles: Vec<Vec<char>>, moves: &[(usize, usize, usize)]) -> String {
    for &(n, f, t) in moves {
        let at = piles[f - 1].len() - n;
        let mut l = piles[f - 1].split_off(at);
        piles[t - 1].append(&mut l);
    }
    piles.iter_mut().map(|r| r.pop().unwrap()).collect()
}

fn main() {
    let (piles, moves) = read_input();
    println!("Task 1: {}", task_one(piles.clone(), &moves[..]));
    println!("Task 2: {}", task_two(piles, &moves[..]));
}
