use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum State {
    None,
    Empty,
    Wall,
}

pub fn main() {
    let pattern = Regex::new(r"(R|L|\d+)").unwrap();
    let (map_data, mut move_data) = include_str!("./input.txt").split_once("\n\n").unwrap();

    let map = " "
        .lines()
        .chain(map_data.lines())
        .chain(" ".lines())
        .map(|l| {
            " ".chars()
                .chain(l.chars())
                .chain(" ".chars())
                .map(|c| match c {
                    '.' => State::Empty,
                    '#' => State::Wall,
                    _ => State::None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut direction = 0;

    let mut y = 1;
    let mut x = map[y].iter().position(|s| *s == State::Empty).unwrap();

    while move_data.len() > 0 {
        let mat = pattern.find(move_data).unwrap();
        let (part, end) = move_data.split_at(mat.end());
        move_data = end;

        if let Ok(mut i) = part.parse::<usize>() {
            while i > 0 {
                let mut x2 = x;
                let mut y2 = y;

                match direction {
                    0 => x2 += 1,
                    1 => y2 += 1,
                    2 => x2 -= 1,
                    _ => y2 -= 1,
                }

                if Some(&State::None) == map.get(y2).and_then(|l| l.get(x2)).or(Some(&State::None))
                {
                    match direction {
                        0 => {
                            x2 = map
                                .get(y2)
                                .unwrap()
                                .iter()
                                .position(|s| *s != State::None)
                                .unwrap()
                        }
                        1 => {
                            y2 = map
                                .iter()
                                .map(|l| l.get(x2).or(Some(&State::None)).unwrap())
                                .position(|s| *s != State::None)
                                .unwrap()
                        }
                        2 => {
                            x2 = map
                                .get(y2)
                                .unwrap()
                                .iter()
                                .rposition(|s| *s != State::None)
                                .unwrap()
                        }
                        _ => {
                            y2 = map
                                .iter()
                                .map(|l| l.get(x2).or(Some(&State::None)).unwrap())
                                .rposition(|s| *s != State::None)
                                .unwrap()
                        }
                    }
                }

                match map.get(y2).and_then(|l| l.get(x2)) {
                    Some(&State::Wall) => {
                        break;
                    }
                    Some(&State::Empty) => {
                        x = x2;
                        y = y2;
                    }
                    _ => {
                        println!("{x2} {y2} {direction}");
                        panic!();
                    }
                }

                i -= 1;
            }
        } else {
            if part == "R" {
                direction += 1;
                direction %= 4;
            } else {
                if direction == 0 {
                    direction = 3;
                } else {
                    direction -= 1;
                }
            }
        }
    }

    println!("{}", y * 1000 + x * 4 + direction);
}
