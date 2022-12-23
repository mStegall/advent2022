use std::collections::{HashMap, HashSet, VecDeque};

pub fn main() {
    let mut elfs = HashSet::new();

    for (y, line) in include_str!("./input.txt").lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elfs.insert((x as isize, y as isize));
            }
        }
    }

    println!("{elfs:?}");

    let mut moves: VecDeque<([(isize, isize); 3], (isize, isize))> = VecDeque::new();
    moves.push_back(([(-1, -1), (0, -1), (1, -1)], (0, -1)));
    moves.push_back(([(-1, 1), (0, 1), (1, 1)], (0, 1)));
    moves.push_back(([(-1, -1), (-1, 0), (-1, 1)], (-1, 0)));
    moves.push_back(([(1, -1), (1, 0), (1, 1)], (1, 0)));

    for _ in 0..10 {
        let mut proposed: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
        let mut destinations: HashMap<(isize, isize), usize> = HashMap::new();

        for (x, y) in elfs.iter() {
            if [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
            .iter()
            .any(|(dx, dy)| elfs.contains(&(x + dx, y + dy)))
            {
                for (c, (dx, dy)) in moves.iter() {
                    if c.iter().all(|(dx, dy)| !elfs.contains(&(x + dx, y + dy))) {
                        proposed.insert((*x, *y), (x + dx, y + dy));
                        destinations
                            .entry((x + dx, y + dy))
                            .and_modify(|s| *s += 1)
                            .or_insert(1);
                        break;
                    }
                }
            }
        }

        for (s, d) in proposed.iter() {
            if *destinations.get(d).unwrap() == 1 {
                elfs.remove(s);
                elfs.insert(*d);
            }
        }

        // println!("{elfs:?}");
        let left = elfs.iter().map(|(x, _)| x).min().unwrap();
        let right = elfs.iter().map(|(x, _)| x).max().unwrap();
        let top = elfs.iter().map(|(_, y)| y).min().unwrap();
        let bottom = elfs.iter().map(|(_, y)| y).max().unwrap();

        print!("\n{top},{bottom},{left},{right}\n");
        for y in *top..*bottom + 1 {
            for x in *left..*right + 1 {
                if elfs.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n")
        }

        let t = moves.pop_front().unwrap();
        moves.push_back(t);
    }

    let left = elfs.iter().map(|(x, _)| x).min().unwrap();
    let right = elfs.iter().map(|(x, _)| x).max().unwrap();
    let top = elfs.iter().map(|(_, y)| y).min().unwrap();
    let bottom = elfs.iter().map(|(_, y)| y).max().unwrap();

    println!("{}", (right - left +1) * (bottom - top+1) - elfs.len() as isize);
}
