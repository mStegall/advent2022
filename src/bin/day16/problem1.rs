use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Valve {
    flow: usize,
    tunnels: Vec<String>,
}

pub fn main() {
    let pattern =
        Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();

    let mut valves: HashMap<String, Valve> = HashMap::new();

    for line in include_str!("./test.txt").lines() {
        let d = pattern.captures(line).unwrap();
        let rate = d[2].parse::<usize>().unwrap();
        let tunnels = d[3].split(", ").map(|s| s.to_string()).collect::<Vec<_>>();

        valves.insert(
            d[1].to_string(),
            Valve {
                flow: rate,
                tunnels: tunnels,
            },
        );
    }

    let mut states_seen: HashMap<(Vec<String>, String, usize), usize> = HashMap::new();

    let mut states_available: Vec<(Vec<String>, String, usize, usize)> =
        vec![(vec![], "AA".to_string(), 30, 0)];

    while states_available.len() > 0 {
        let s = states_available.pop().unwrap();
        let v = valves.get(&s.1).unwrap();

        let released =
            s.0.iter()
                .map(|k| valves.get(k).unwrap().flow)
                .sum::<usize>();
        let n_released = s.3 + released;

        if s.2 == 0 {
            continue;
        }

        if v.flow != 0 && !s.0.contains(&s.1) {
            let mut ns = s.0.clone();
            ns.push(s.1.clone());
            let n = (ns.clone(), s.1.clone(), s.2 - 1, n_released);
            let k = (ns.clone(), s.1.clone(), s.2 - 1);
            if let Some(p) = states_seen.get(&k) {
                if n_released > *p {
                    states_seen.insert(k, n_released);
                    states_available.push(n);
                }
            } else {
                states_seen.insert(k, n_released);
                states_available.push(n);
            }
        }
        for t in v.tunnels.iter() {
            let n = (s.0.clone(), t.clone(), s.2 - 1, n_released);
            let k = (s.0.clone(), t.clone(), s.2 - 1);
            if let Some(p) = states_seen.get(&k) {
                if n_released > *p {
                    states_seen.insert(k, n_released);
                    states_available.push(n);
                }
            } else {
                states_seen.insert(k, n_released);
                states_available.push(n);
            }
        }
    }

    println!("{}", states_seen.values().max().unwrap());
}
