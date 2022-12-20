use regex::Regex;
use std::hash::Hash;

#[derive(Debug, Hash)]
struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    time_remaining: usize,
    could_buy: [bool; 4],
}

pub fn main() {
    let pattern = Regex::new(r"\d+").unwrap();
    let mut lines = include_str!("./test.txt").lines().collect::<Vec<_>>();
    lines.truncate(1);

    let data = lines
        .iter()
        .map(|l| {
            let nums = pattern
                .captures_iter(l)
                .filter_map(|i| i[0].parse::<usize>().ok())
                .collect::<Vec<_>>();

            Blueprint {
                id: nums[0],
                ore: nums[1],
                clay: nums[2],
                obsidian: (nums[3], nums[4]),
                geode: (nums[5], nums[6]),
            }
        })
        .collect::<Vec<_>>();

    let states = data
        .iter()
        .map(|bp| {
            let mut final_states: Vec<State> = vec![];
            let mut states: Vec<State> = vec![State {
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                time_remaining: 32,
                could_buy: [false; 4],
            }];

            let mut best = [0; 33];

            while let Some(s) = states.pop() {
                let mut base = s.clone();
                let mut p = vec![];

                if bp.geode.0 <= s.ore && bp.geode.1 <= s.obsidian && !s.could_buy[3] {
                    let mut ss = s.clone();
                    ss.ore -= bp.geode.0;
                    ss.obsidian -= bp.geode.1;
                    ss.geode_robots += 1;

                    ss.could_buy = [false; 4];
                    base.could_buy[3] = true;

                    p.push(ss);
                } else {
                    if bp.obsidian.0 <= s.ore && bp.obsidian.1 <= s.clay && !s.could_buy[2] {
                        let mut ss = s.clone();
                        ss.ore -= bp.obsidian.0;
                        ss.clay -= bp.obsidian.1;
                        ss.obsidian_robots += 1;

                        ss.could_buy = [false; 4];
                        base.could_buy[2] = true;

                        p.push(ss);
                    }
                    if bp.clay <= s.ore && !s.could_buy[1] {
                        let mut ss = s.clone();
                        ss.ore -= bp.clay;
                        ss.clay_robots += 1;

                        ss.could_buy = [false; 4];
                        base.could_buy[1] = true;

                        p.push(ss);
                    }
                    if bp.ore <= s.ore && !s.could_buy[0] {
                        let mut ss = s.clone();
                        ss.ore -= bp.ore;
                        ss.ore_robots += 1;
                        ss.could_buy = [false; 4];
                        base.could_buy[0] = true;
                        p.push(ss);
                    }
                }

                p.push(base);

                while let Some(mut ss) = p.pop() {
                    ss.ore += s.ore_robots;
                    ss.clay += s.clay_robots;
                    ss.obsidian += s.obsidian_robots;
                    ss.geode += s.geode_robots;
                    ss.time_remaining -= 1;

                    if ss.time_remaining == 0 {
                        final_states.push(ss)
                    } else {
                        if best[ss.time_remaining]
                            > ss.geode + (ss.time_remaining + 1) * (ss.time_remaining + 1)
                        {
                            continue;
                        } else if best[ss.time_remaining] < ss.geode {
                            best[ss.time_remaining] = ss.geode;
                        }
                        states.push(ss)
                    }
                }
            }

            final_states.iter().map(|s| s.geode).max().unwrap()
        })
        .collect::<Vec<_>>();

    println!("{states:?}");
}
