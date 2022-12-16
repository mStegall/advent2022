use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Valve {
    flow: usize,
    tunnels: Vec<String>,
}

pub fn main() {
    let pattern =
        Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();

    let mut valves: HashMap<String, Valve> = HashMap::new();

    let mut useful_valves = 0;

    for line in include_str!("./test.txt").lines() {
        let d = pattern.captures(line).unwrap();
        let rate = d[2].parse::<usize>().unwrap();
        let tunnels = d[3].split(", ").map(|s| s.to_string()).collect::<Vec<_>>();

        if rate > 0 {
            useful_valves += 1;
        }

        valves.insert(
            d[1].to_string(),
            Valve {
                flow: rate,
                tunnels: tunnels,
            },
        );
    }

    let mut states_seen: HashMap<(Vec<String>, String, String, usize), usize> = HashMap::new();
    states_seen.insert((vec![], "AA".to_string(), "AA".to_string(), 26), 0);

    let mut states_available: VecDeque<(Vec<String>, String, String, usize, usize)> =
        VecDeque::new();
    states_available.push_front((vec![], "AA".to_string(), "AA".to_string(), 26, 0));

    while states_available.len() > 0 {
        let s = states_available.pop_front().unwrap();
        // println!("running: {s:?}");
        if let Some(p) = states_seen.get(&(s.0.clone(), s.1.clone(), s.2.clone(), s.3)) {
            if *p != s.4 {
                // println!("bailing");
                continue;
            }
        }

        let released =
            s.0.iter()
                .map(|k| valves.get(k).unwrap().flow)
                .sum::<usize>();
        let n_released = s.4 + released;

        if s.3 == 0 {
            // println!("its over");

            continue;
        }

        if s.0.len() == useful_valves {
            let k = (s.0.clone(), s.1.clone(), s.2.clone(), 0);
            let f = s.4 + released * (s.3);

            if let Some(p) = states_seen.get(&k) {
                if f > *p {
                    states_seen.insert(k, f);
                }
            } else {
                states_seen.insert(k, f);
            }

            continue;
        }

        let v = valves.get(&s.1).unwrap();

        let mut v_inter: Vec<(Vec<String>, String, String, usize, usize)> = vec![];

        if v.flow != 0 && !s.0.contains(&s.1) {
            let mut ns = s.0.clone();
            ns.push(s.1.clone());
            let n = (ns.clone(), s.1.clone(), s.2.clone(), s.3 - 1, n_released);
            let k = (ns.clone(), s.1.clone(), s.2.clone(), s.3 - 1);
            if let Some(p) = states_seen.get(&k) {
                if n_released > *p {
                    states_seen.insert(k, n_released);
                    v_inter.push(n);
                }
            } else {
                states_seen.insert(k, n_released);
                v_inter.push(n);
            }
        }
        for t in v.tunnels.iter() {
            let n = (s.0.clone(), t.clone(), s.2.clone(), s.3 - 1, n_released);
            let k = (s.0.clone(), t.clone(), s.2.clone(), s.3 - 1);
            if let Some(p) = states_seen.get(&k) {
                if n_released > *p {
                    states_seen.insert(k, n_released);
                    v_inter.push(n);
                }
            } else {
                states_seen.insert(k, n_released);
                v_inter.push(n);
            }
        }

        // println!("me: {v_inter:?}");

        for t in v_inter.iter() {
            let w = valves.get(&t.2).unwrap();

            if w.flow != 0 && !t.0.contains(&t.2) {
                let mut ns = t.0.clone();
                ns.push(t.2.clone());
                let n = (ns.clone(), t.1.clone(), t.2.clone(), t.3, n_released);
                let k = (ns.clone(), t.1.clone(), t.2.clone(), t.3);
                if let Some(p) = states_seen.get(&k) {
                    if n_released > *p {
                        // println!("elephant: {n:?}");
                        states_seen.insert(k, n_released);
                        states_available.push_back(n);
                    }
                } else {
                    // println!("elephant: {n:?}");

                    states_seen.insert(k, n_released);
                    states_available.push_back(n);
                }
            }
            for u in w.tunnels.iter() {
                let n = (t.0.clone(), t.1.clone(), u.clone(), t.3, n_released);
                let k = (t.0.clone(), t.1.clone(), u.clone(), t.3);
                if let Some(p) = states_seen.get(&k) {
                    if n_released > *p {
                        // println!("elephant: {n:?}");

                        states_seen.insert(k, n_released);
                        states_available.push_back(n);
                    }
                } else {
                    // println!("elephant: {n:?}");

                    states_seen.insert(k, n_released);
                    states_available.push_back(n);
                }
            }
        }
    }

    // println!("{states_seen:?}");
    println!("{}", states_seen.values().max().unwrap());
}
