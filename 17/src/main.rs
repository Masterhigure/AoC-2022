use std::{
    collections::{HashMap, HashSet},
    fs,
};

type Input = (Vec<i64>, Vec<Vec<(i64, i64)>>);

#[derive(Debug)]
struct Cave {
    rocks: HashSet<(i64, i64)>,
}

impl Cave {
    fn intersects(&self, other: &[(i64, i64)]) -> bool {
        !self.rocks.is_disjoint(&other.iter().cloned().collect())
            || other.iter().any(|&(x, y)| !(0..7).contains(&x) || y < 0)
    }
}

pub fn read_input(filename: &str) -> Input {
    let wind = fs::read(filename)
        .unwrap()
        .iter()
        .filter_map(|&u| {
            if u == b'<' {
                Some(-1)
            } else if u == b'>' {
                Some(1)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let rocks = fs::read_to_string("rocks.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.rsplit('\n')
                .enumerate()
                .flat_map(|(y, r)| {
                    r.bytes().enumerate().filter_map(move |(x, u)| {
                        if u == b'#' {
                            Some((x as i64, y as i64))
                        } else {
                            None
                        }
                    })
                })
                .collect()
        })
        .collect();
    (wind, rocks)
}

pub fn task_one(i: Input) -> i64 {
    // Pure brute force tetris. Not much interesting going on here.
    let (wind, rocks) = i;
    let mut cave = Cave {
        rocks: HashSet::new(),
    };
    let mut top = -1;
    let mut windex = 0;
    for k in 0..2022 {
        let mut falling = rocks[k % rocks.len()].clone();
        for (_, y) in falling.iter_mut() {
            *y += top + 4;
        }
        loop {
            let gust = wind[windex % wind.len()];
            windex += 1;
            for (x, _) in falling.iter_mut() {
                *x += gust;
            }
            if cave.intersects(&falling) {
                for (x, _) in falling.iter_mut() {
                    *x -= gust;
                }
            }
            for (_, y) in falling.iter_mut() {
                *y -= 1;
            }
            if cave.intersects(&falling) {
                for (_, y) in falling.iter_mut() {
                    *y += 1;
                }
                cave.rocks.extend(falling.iter());
                top = top.max(falling.iter().map(|(_, y)| *y).max().unwrap());
                break;
            }
        }
    }
    top + 1
}

pub fn task_two(i: Input) -> i64 {
    // Too large for brute force. Adapt part 1 to look for a repeating period,
    // find number of stones and height of said period, find where in said
    // period 1e12 lands, continue tetrising until there is a whole number
    // of periods until 1e12, do the arithmetic to find total height.
    let (wind, rocks) = i;
    let mut cave = Cave {
        rocks: HashSet::new(),
    };
    let mut top = -1;
    let mut windex = 0;

    // Additional variables needed for period search
    let mut repeat_indices = HashMap::new(); // Indices from which inputs and rocks both repeat
    let mut phase1 = true; // phase 1: look for period. phase 2: period found, look for right spot
    let mut stones_in_period = 0;
    let mut height_of_period = 0;
    let mut final_target = 0; // From this rock there is a whole number of periods until 1e12
    for k in 0.. {
        let mut falling = rocks[k % rocks.len()].clone();
        for (_, y) in falling.iter_mut() {
            *y += top + 4;
        }
        loop {
            let gust = wind[windex % wind.len()];
            windex += 1;
            for (x, _) in falling.iter_mut() {
                *x += gust;
            }
            if cave.intersects(&falling) {
                for (x, _) in falling.iter_mut() {
                    *x -= gust;
                }
            }
            for (_, y) in falling.iter_mut() {
                *y -= 1;
            }
            if cave.intersects(&falling) {
                for (_, y) in falling.iter_mut() {
                    *y += 1;
                }
                cave.rocks.extend(falling.iter());
                top = top.max(falling.iter().map(|(_, y)| *y).max().unwrap());

                // Check to see if we have entered a repetition
                if phase1 {
                    // Have we seen this exact combination of input and rock indices before?
                    if let Some(&(prev, s)) =
                        repeat_indices.get(&(windex % wind.len(), k % rocks.len()))
                    {
                        // Check to see whether the rock pattern is the same.
                        // 100 and 2000 are picked arbitrarily, but some strictly positive number
                        // in place of 100 is necessary as the next rock may fall below the
                        // current one and mess with the immediate vincinity.
                        // 2000 seemed overkill enough to pick up
                        // any non-periodicity.
                        let last = cave
                            .rocks
                            .iter()
                            .filter(|(_, y)| *y < prev - 100 && *y > prev - 2000)
                            .map(|&(x, y)| (x, y - prev))
                            .collect::<HashSet<_>>();
                        let now = cave
                            .rocks
                            .iter()
                            .filter(|&(_, y)| *y < top - 100 && *y > top - 2000)
                            .map(|&(x, y)| (x, y - top))
                            .collect::<HashSet<_>>();
                        if last == now {
                            // Period found! Initiate phase 2
                            stones_in_period = k - s;
                            height_of_period = top - prev;
                            final_target = (1_000_000_000_000 - k) % stones_in_period + k;
                            phase1 = false;
                        }
                    } else {
                        repeat_indices.insert((windex % wind.len(), k % rocks.len()), (top, k));
                    }
                } else if k == final_target {
                    // And we are finally done!
                    return top
                        + height_of_period
                            * ((1_000_000_000_000 - k as i64) / stones_in_period as i64);
                }
                break;
            }
        }
    }
    0
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
        assert_eq!(task_one(input), 3068);
    }

    #[test]
    fn test_two() {
        let input = read_input("example.txt");
        assert_eq!(task_two(input), 1514285714288);
    }
}
