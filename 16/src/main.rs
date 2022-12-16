#![allow(unused)]
use std::{collections::HashMap, fs, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Valve {
    name: String,
    flow: i32,
    neighs: Vec<String>,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.chars().skip(6).take(2).collect();
        let flow = s.split(['=', ';']).nth(1).unwrap().parse().unwrap();
        let neighs = s
            .split_once("valve")
            .unwrap()
            .1
            .split_at(1)
            .1
            .trim_start()
            .split(", ")
            .map(str::to_string)
            .collect();
        Ok(Valve { name, flow, neighs })
    }
}

impl Ord for Valve {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.flow.cmp(&other.flow)
    }
}

impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn read_input() -> HashMap<String, Valve> {
    fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.parse::<Valve>().unwrap())
        .map(|v| (v.name.clone(), v))
        .collect()
}

fn best_value_1(
    time: i32,
    pos: String,
    unused: &mut HashMap<String, Valve>,
    whole: &HashMap<String, Valve>,
) -> (i32, Vec<(String, i32)>) {
    if time >= 26 {
        return (0, vec![(pos, time)]);
    }
    let mut best = 0;
    let mut best_path = Vec::new();
    let mut depth = 0;
    let mut neighs = whole.get(&pos).unwrap().neighs.clone();
    let mut cave_map: HashMap<String, i32> = HashMap::new();
    while !neighs.is_empty() {
        depth += 1;
        for s in &neighs {
            cave_map.insert(s.clone(), depth);
        }
        neighs = neighs
            .iter()
            .flat_map(|s| whole.get(s).unwrap().neighs.iter())
            .filter(|&s| !cave_map.contains_key(s))
            .cloned()
            .collect::<Vec<_>>();
        neighs.sort();
        neighs.dedup();
    }
    let unused_names = unused.keys().cloned().collect::<Vec<_>>();
    for n in unused_names {
        if let Some(v) = unused.remove(&n) {
            let new_time = time + *cave_map.get(&n).unwrap() + 1;
            let result = best_value_1(new_time, n.clone(), unused, whole);
            let b = result.0 + v.flow * (30 - new_time);
            if b > best {
                best = b;
                best_path = result.1;
                best_path.push((n.clone(), new_time));
            }
            unused.insert(n, v);
        } else {
            unreachable!()
        }
    }
    (best, best_path)
}

fn best_value_2(
    time: i32,
    pos: String,
    other_pos: (String, i32),
    unused: &mut HashMap<String, Valve>,
    so_far: i32,
    best_so_far: i32,
    cave_map: &HashMap<(String, String), i32>,
) -> i32 {
    if time >= 26 {
        return 0;
    }
    let mut best = 0;
    let mut depth = 0;

    // Pruning. See comment in best_value_2 for details on this upper estimate.
    let mut best_remaining = unused.values().map(|v| v.flow).collect::<Vec<_>>();
    best_remaining.sort();
    let mut best_possible = 0;
    for trial_time in (time + 1)..=26 {
        if best_remaining.is_empty() {
            break;
        }
        if (trial_time + time) % 2 == 0 {
            best_possible += best_remaining.pop().unwrap() * (26 - trial_time);
        }
        if best_remaining.is_empty() {
            break;
        }
        if trial_time - time >= other_pos.1 && (trial_time + time + other_pos.1) % 2 == 0 {
            best_possible += best_remaining.pop().unwrap() * (26 - trial_time);
        }
    }
    if so_far + best_possible <= best_so_far {
        return best_possible;
    }

    // Recursive tree traversal time!
    let mut bsf = best_so_far;
    let unused_names = unused.keys().cloned().collect::<Vec<_>>();
    for n in unused_names {
        //println!("Time: {}, looking at {}", time, n);
        if let Some(v) = unused.remove(&n) {
            let time_spent = *cave_map.get(&(pos.clone(), n.clone())).unwrap() + 1;
            let mut b = 0;
            let flow = v.flow * (26 - time - time_spent);
            if time_spent > other_pos.1 {
                b = best_value_2(
                    time + other_pos.1,
                    other_pos.0.clone(),
                    (n.clone(), time_spent - other_pos.1),
                    unused,
                    so_far + flow,
                    bsf,
                    cave_map,
                ) + flow;
            } else {
                b = best_value_2(
                    time + time_spent,
                    n.clone(),
                    (other_pos.0.clone(), other_pos.1 - time_spent),
                    unused,
                    so_far + flow,
                    bsf,
                    cave_map,
                ) + flow;
            }
            if b > best {
                // n is so far the best valve to go to next
                best = b;
            }
            if b > bsf {
                // the path through here and then to n is better than any
                // path previously encountered. Update pruning parameter!
                bsf = b;
            }
            unused.insert(n, v);
        } else {
            unreachable!()
        }
    }
    best
}

fn task_one(i: &HashMap<String, Valve>) -> i32 {
    let mut unused: HashMap<String, Valve> = i
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    best_value_1(0, "AA".to_string(), &mut unused, i).0
}

fn task_two(i: &HashMap<String, Valve>) -> i32 {
    let mut unused: HashMap<String, Valve> = i
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    // It turns out to be 5 times faster to close in on the end
    // result from above. So here is a simple upper bound for the final result.
    // Assuming both you and the elephant get to the best remaining valve
    // in a single minute, then spend a minute turning it on,
    // this calculation finds the resulting score.
    // (The rearrangement inequality ensures that greedy is indeed best.)
    // Same upper bound idea is used for pruning inside the recursion.
    let mut flows = unused.values().map(|v| v.flow).collect::<Vec<_>>();
    flows.sort();
    let mut quick_estimate = 0;
    for time in 1..26 {
        if time % 2 == 0 {
            for _ in 0..2 {
                if let Some(f) = flows.pop() {
                    quick_estimate += (26 - time) * f;
                }
            }
        }
    }

    // Distance graph for all valves with flow > 0. In part 1,
    // I calculated all distances as needed, but
    // in part 2, this becomes very expensive, increasing run time by
    // a factor of roundabout 20.
    let mut cave_map = HashMap::new();
    for pos in unused.keys().chain(Some(&("AA".to_string()))) {
        let mut depth = 0;
        // Make a full list of possible nodes to go to next and the distance to them
        let mut neighs = i.get(pos).unwrap().neighs.clone();
        while !neighs.is_empty() {
            depth += 1;
            for s in &neighs {
                cave_map.insert((pos.clone(), s.clone()), depth);
            }
            neighs = neighs
                .iter()
                .flat_map(|s| i.get(s).unwrap().neighs.iter())
                .filter(|&s| !cave_map.contains_key(&(pos.clone(), s.clone())))
                .cloned()
                .collect::<Vec<_>>();
            neighs.sort();
            neighs.dedup();
        }
    }

    // Closing in on the result from above, exploiting the pruning
    // to quickly whittle our way down to an underestimate,
    // which finally lets the algorithm work out an answer
    loop {
        let result = best_value_2(
            0,                     // time spent so far
            "AA".to_string(),      // Current position of one character
            ("AA".to_string(), 0), // Current position of the other character, and time to goal
            &mut unused,           // Yet unturned valves
            0,                     // Total pressure released by currently open valves
            quick_estimate,        // Best result so far, for pruning
            &cave_map,             // Distance graph between all nodes with useful valve
        );
        if result > quick_estimate {
            return result;
        }
        quick_estimate -= 400;
    }
    0
}

fn main() {
    let input = read_input();
    println!("Task 1: {}", task_one(&input));
    println!("Task 2: {}", task_two(&input));
}
