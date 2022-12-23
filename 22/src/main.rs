#![allow(unused)]
use std::fs;
use std::io::stdin;

type Input = (Vec<Vec<u8>>, Vec<i32>, Vec<i32>);

fn visualise(i: &[Vec<u8>], x: usize, y: usize) {
    for l in 0..(i.len()) {
        let mut line = "".to_string();
        for c in 0..(i[0].len()) {
            let o = match i[l][c] {
                0 => ' ',
                1 => '#',
                2 => '.',
                _ => unreachable!(),
            };
            if l == y && c == x {
                line.push('X');
            } else {
                line.push(o);
            }
        }
        println!("{line}");
    }
}

pub fn read_input(filename: &str) -> Input {
    let raw_input = fs::read_to_string(filename).unwrap();
    let input = raw_input.split_once("\n\n").unwrap();
    let mut i0 = input
        .0
        .split('\n')
        .map(|c| c.bytes().map(|b| b / 11 - 2).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let i1 = input
        .1
        .split(|c: char| c.is_alphabetic())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let mut i2 = input
        .1
        .bytes()
        .filter_map(|b| match b {
            b'R' => Some(1),
            b'L' => Some(-1),
            _ => None,
        })
        .collect::<Vec<_>>();
    let max_width = i0.iter().map(|v| v.len()).max().unwrap();
    i2.push(0);
    i0.iter_mut().for_each(|v| v.resize(max_width, 0));
    (i0, i1, i2)
}

pub fn task_one(mut i: Input) -> i32 {
    let map = i.0;
    let steps = i.1;
    let turns = i.2;
    let mut rot = 0;
    let mut x = map[0].iter().position(|&b| b == 2).unwrap();
    let mut y = 0;
    let wait = stdin();
    let mut s = "".to_string();
    for (&m, &r) in steps.iter().zip(turns.iter()) {
        for _ in 0..m {
            match rot {
                0 => {
                    if x < map[y].len() - 1 && map[y][x + 1] == 2 {
                        x += 1;
                    } else if (x < map[y].len() - 1 && map[y][x + 1] == 0) || x + 1 >= map[y].len()
                    {
                        let nx = map[y].iter().position(|&b| b != 0).unwrap();
                        if map[y][nx] == 2 {
                            x = nx;
                        }
                    }
                }
                1 => {
                    if y < map.len() - 1 && map[y + 1][x] == 2 {
                        y += 1;
                    } else if (y < map.len() - 1 && map[y + 1][x] == 0) || y + 1 >= map.len() {
                        let ny = map.iter().position(|v| v[x] != 0).unwrap();
                        if map[ny][x] == 2 {
                            y = ny;
                        }
                    }
                }
                2 => {
                    if x > 0 && map[y][x - 1] == 2 {
                        x -= 1;
                    } else if (x > 0 && map[y][x - 1] == 0) || x == 0 {
                        let nx =
                            map[y].len() - 1 - map[y].iter().rev().position(|&b| b != 0).unwrap();
                        if map[y][nx] == 2 {
                            x = nx;
                        }
                    }
                }
                3 => {
                    if y > 0 && map[y - 1][x] == 2 {
                        y -= 1;
                    } else if (y > 0 && map[y - 1][x] == 0) || y == 0 {
                        let ny = map.len() - 1 - map.iter().rev().position(|v| v[x] != 0).unwrap();
                        if map[ny][x] == 2 {
                            y = ny;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        rot += r + 4;
        rot %= 4;
    }
    1000 * (y as i32 + 1) + 4 * (x as i32 + 1) + rot
}

fn next_coord(x: usize, y: usize, rot: i32) -> (usize, usize, i32) {
    let fx = x / 50;
    let fy = y / 50;
    match (fx, fy, rot) {
        (2, 0, 1) => (y + 50, x - 50, 2),
        (1, 1, 0) => (y + 50, x - 50, 3),
        (1, 1, 2) => (y - 50, x + 50, 1),
        (0, 2, 3) => (y - 50, x + 50, 0),
        (1, 2, 1) => (y - 100, x + 100, 2),
        (0, 3, 0) => (y - 100, x + 100, 3),
        (0, 2, 2) => (x + 50, 149 - y, 0),
        (1, 0, 2) => (x - 50, 149 - y, 2),
        (1, 0, 3) => (y, x + 100, 0),
        (0, 3, 2) => (y - 100, x, 1),
        (1, 2, 0) => (x + 50, 149 - y, 2),
        (2, 0, 0) => (x - 50, 149 - y, 2),
        (0, 3, 1) => (x + 100, y - 199, 3),
        (2, 0, 3) => (x - 100, y + 199, 3),
        _ => todo!(),
    }
}

pub fn task_two(i: Input) -> i32 {
    let map = i.0;
    let steps = i.1;
    let turns = i.2;
    let mut rot = 0;
    let mut x = map[0].iter().position(|&b| b == 2).unwrap();
    let mut y = 0;
    let wait = stdin();
    let mut s = "".to_string();
    for (&m, &r) in steps.iter().zip(turns.iter()) {
        for _ in 0..m {
            match rot {
                0 => {
                    if x < map[y].len() - 1 && map[y][x + 1] == 2 {
                        x += 1;
                    } else if (x < map[y].len() - 1 && map[y][x + 1] == 0) || x + 1 >= map[y].len()
                    {
                        let (nx, ny, nr) = next_coord(x, y, rot);
                        if map[ny][nx] == 2 {
                            x = nx;
                            y = ny;
                            rot = nr;
                        }
                    }
                }
                1 => {
                    if y < map.len() - 1 && map[y + 1][x] == 2 {
                        y += 1;
                    } else if (y < map.len() - 1 && map[y + 1][x] == 0) || y + 1 >= map.len() {
                        let (nx, ny, nr) = next_coord(x, y, rot);
                        if map[ny][nx] == 2 {
                            x = nx;
                            y = ny;
                            rot = nr;
                        }
                    }
                }
                2 => {
                    if x > 0 && map[y][x - 1] == 2 {
                        x -= 1;
                    } else if (x > 0 && map[y][x - 1] == 0) || x == 0 {
                        let (nx, ny, nr) = next_coord(x, y, rot);
                        if map[ny][nx] == 2 {
                            x = nx;
                            y = ny;
                            rot = nr;
                        }
                    }
                }
                3 => {
                    if y > 0 && map[y - 1][x] == 2 {
                        y -= 1;
                    } else if (y > 0 && map[y - 1][x] == 0) || y == 0 {
                        let (nx, ny, nr) = next_coord(x, y, rot);
                        if map[ny][nx] == 2 {
                            x = nx;
                            y = ny;
                            rot = nr;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        rot += r + 4;
        rot %= 4;
    }
    1000 * (y as i32 + 1) + 4 * (x as i32 + 1) + rot
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
    fn test_one() {
        let input = read_input("example.txt");
        assert_eq!(task_one(input), 6032);
    }

    #[test]
    fn test_two() {
        let input = read_input("example.txt");
        assert_eq!(task_two(input), 5031);
    }
}
