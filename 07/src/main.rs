#![allow(unused)]
use std::{
    collections::{hash_map::Entry, HashMap},
    fs,
};

#[derive(Debug)]
struct Dir {
    path: String,
    sub_dirs: Vec<String>,
    files: Vec<String>,
}

impl Dir {
    fn new(path: String) -> Self {
        Dir {
            path,
            sub_dirs: vec![],
            files: vec![],
        }
    }

    fn size(&self, dirs: &HashMap<String, Dir>, files: &HashMap<String, i32>) -> i32 {
        self.sub_dirs
            .iter()
            .map(|s| {
                dirs.get(&(self.path.clone() + "/" + s))
                    .unwrap()
                    .size(dirs, files)
            })
            .sum::<i32>()
            + self
                .files
                .iter()
                .map(|s| files.get(&(self.path.clone() + "/" + s)).unwrap())
                .sum::<i32>()
    }
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input_7")
        .unwrap()
        .split('\n')
        .filter(|&s| !s.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>()
}

fn parse_input(i: Vec<String>) -> (HashMap<String, Dir>, HashMap<String, i32>) {
    let (mut dirs, mut files): (HashMap<String, Dir>, _) = (HashMap::new(), HashMap::new());
    dirs.insert("/".to_owned(), Dir::new("/".into()));
    let mut current = "/".to_string();
    let mut depth = 0;
    for line in i[1..].iter() {
        let line = line.split_whitespace().collect::<Vec<_>>();
        match line[0] {
            "$" => match line[1] {
                "cd" => match line[2] {
                    ".." => {
                        current = current.rsplit_once('/').unwrap().0.to_string();
                    }
                    d => {
                        current = current + "/" + d;
                    }
                },
                "ls" => {}
                _ => {
                    unreachable!()
                }
            },
            "dir" => {
                let path = current.clone() + "/" + line[1];
                if let Entry::Vacant(e) = dirs.entry(path) {
                    e.insert(Dir::new(current.clone() + "/" + line[1]));
                    dirs.get_mut(&current)
                        .unwrap()
                        .sub_dirs
                        .push(line[1].into());
                }
            }
            s => {
                if files
                    .insert(current.clone() + "/" + line[1], s.parse().unwrap())
                    .is_none()
                {
                    dirs.get_mut(&current).unwrap().files.push(line[1].into());
                }
            }
        }
    }
    (dirs, files)
}

fn task_one(dirs: &HashMap<String, Dir>, files: &HashMap<String, i32>) -> i32 {
    dirs.values()
        .filter(|&v| v.size(dirs, files) <= 100_000)
        .map(|v| v.size(dirs, files))
        .sum()
}

fn task_two(dirs: &HashMap<String, Dir>, files: &HashMap<String, i32>) -> i32 {
    let used = dirs.get("/").unwrap().size(dirs, files);
    let free = 70000000 - used;
    let needed = 30000000 - free;
    dirs.values()
        .filter_map(|d| match d.size(dirs, files) {
            n if n >= needed => Some(n),
            _ => None,
        })
        .min()
        .unwrap()
}

fn main() {
    let input = read_input();
    let (dirs, files) = parse_input(input);
    println!("Task 1: {}", task_one(&dirs, &files));
    println!("Task 2: {}", task_two(&dirs, &files));
}
