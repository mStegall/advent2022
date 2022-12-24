use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

type BlizzardState = HashSet<(usize, usize)>;
type BlizzardDirectionState = HashSet<(usize, usize, Direction)>;

pub fn main() {
    let mut states: Vec<(BlizzardState, BlizzardDirectionState)> = vec![];
    let mut state0 = HashSet::new();
    let mut state_0d = HashSet::new();

    let data = include_str!("./input.txt");
    let width = data.lines().next().unwrap().len() - 1;
    let height = data.lines().count() - 1;

    for (y, l) in data.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '<' => {
                    state0.insert((x, y));
                    state_0d.insert((x, y, Direction::West))
                }
                '>' => {
                    state0.insert((x, y));
                    state_0d.insert((x, y, Direction::East))
                }
                '^' => {
                    state0.insert((x, y));
                    state_0d.insert((x, y, Direction::North))
                }
                'v' => {
                    state0.insert((x, y));
                    state_0d.insert((x, y, Direction::South))
                }
                _ => false,
            };
        }
    }

    states.push((state0, state_0d));

    let start_x = data
        .lines()
        .next()
        .unwrap()
        .chars()
        .position(|s| s == '.')
        .unwrap();
    let end_x = data
        .lines()
        .last()
        .unwrap()
        .chars()
        .position(|s| s == '.')
        .unwrap();

    let mut game_states: VecDeque<(usize, usize, usize)> = VecDeque::new();
    game_states.push_back((start_x, 0, 0));

    let movement_directions: Vec<(isize, isize)> = vec![(0, 0), (1, 0), (-1, 0), (0, -1), (0, 1)];

    println!("{height} {end_x}");

    while let Some((x, y, turns)) = game_states.pop_front() {
        if None == states.get(turns + 1) {
            states.push(make_next(&states[turns].1, width, height));
        }

        let b = &states[turns + 1].0;

        for (dx, dy) in movement_directions.iter() {
            let x2 = (x as isize + dx) as usize;
            let y2 = (y as isize + dy) as usize;

            if x2 == end_x && y2 == height {
                println!("{}", turns + 1);
                return;
            }

            if x2 == start_x && y2 == 0 && *dx == 0 && *dy == 0 {
                game_states.push_back((x2, y2, turns + 1));
            }

            if x2 > 0 && x2 < width && y2 > 0 && y2 < height {
                if !b.contains(&(x2, y2)) && !game_states.contains(&(x2, y2, turns + 1)) {
                    game_states.push_back((x2, y2, turns + 1));
                }
            }
        }

        // println!("{game_states:?}");

        if turns > 1000 {
            break;
        }
    }

    // for (i, (b, _)) in states.iter().enumerate() {
    //     println!("minute {i}");
    //     for y in 0..height + 1 {
    //         for x in 0..width + 1 {
    //             if b.contains(&(x, y)) {
    //                 print!("*");
    //             } else {
    //                 print!(".");
    //             }
    //         }

    //         print!("\n");
    //     }
    // }
}

fn make_next(
    b: &BlizzardDirectionState,
    width: usize,
    height: usize,
) -> (BlizzardState, BlizzardDirectionState) {
    let mut next: BlizzardState = HashSet::new();
    let mut next_d: BlizzardDirectionState = HashSet::new();

    for (x, y, d) in b.iter() {
        match d {
            Direction::West => {
                if *x == 1 {
                    next.insert((width - 1, *y));
                    next_d.insert((width - 1, *y, Direction::West));
                } else {
                    next.insert((x - 1, *y));
                    next_d.insert((x - 1, *y, Direction::West));
                }
            }
            Direction::East => {
                if (x + 1) >= width {
                    next.insert((1, *y));
                    next_d.insert((1, *y, Direction::East));
                } else {
                    next.insert((x + 1, *y));
                    next_d.insert((x + 1, *y, Direction::East));
                }
            }
            Direction::North => {
                if *y == 1 {
                    next.insert((*x, height - 1));
                    next_d.insert((*x, height - 1, Direction::North));
                } else {
                    next.insert((*x, y - 1));
                    next_d.insert((*x, y - 1, Direction::North));
                }
            }
            Direction::South => {
                if (y + 1) >= height {
                    next.insert((*x, 1));
                    next_d.insert((*x, 1, Direction::South));
                } else {
                    next.insert((*x, y + 1));
                    next_d.insert((*x, y + 1, Direction::South));
                }
            }
        }
    }

    return (next, next_d);
}
