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
            println!("Going {i} units {direction}");
            while i > 0 {
                let mut x2 = x;
                let mut y2 = y;

                match direction {
                    0 => x2 += 1,
                    1 => y2 += 1,
                    2 => x2 -= 1,
                    _ => y2 -= 1,
                }

                let mut pending_direction = None;

                if Some(&State::None) == map.get(y2).and_then(|l| l.get(x2)).or(Some(&State::None))
                {
                    let x3;
                    let y3;
                    
                    match direction {
                        0 => {
                            if y2 <= 50 {
                                println!("wrapping from 6 to 3");
                                x3 = 100;
                                y3 = 151 - y2;
                                pending_direction = Some(2);
                            } else if y2 <= 100 {
                                println!("wrapping from 4 to 6");
                                x3 = 50 + y2;
                                y3 = 50;
                                pending_direction = Some(3);
                            } else if y2 <= 150 {
                                println!("wrapping from 3 to 6");
                                x3 = 150;
                                y3 = 151 - y2;
                                pending_direction = Some(2);
                            } else {
                                println!("wrapping from 1 to 3");
                                y3 = 150;
                                x3 = y2 - 100;
                                pending_direction = Some(3);
                            }
                        }
                        1 => {
                            if x2 <= 50 {
                                println!("wrapping from 1 to 6");
                                x3 = 100 + x2;
                                y3 = 1;
                            } else if x2 <= 100 {
                                println!("wrapping from 3 to 1");
                                y3 = 100 + x2;
                                x3 = 50;
                                pending_direction = Some(2);
                            } else {
                                println!("wrapping from 6 to 4");
                                y3 = x2 - 50;
                                x3 = 100;
                                pending_direction = Some(2);
                            }
                        }
                        2 => {
                            if y2 <= 50 {
                                println!("wrapping from 5 to 2");
                                y3 = 151 - y2;
                                x3 = 1;
                                pending_direction = Some(0);
                            } else if y2 <= 100 {
                                println!("wrapping from 4 to 2");
                                x3 = y2 - 50;
                                y3 = 101;
                                pending_direction = Some(1);
                            } else if y2 <= 150 {
                                println!("wrapping from 2 to 5");
                                y3 = 151 - y2;
                                x3 = 51;
                                pending_direction = Some(0);
                            } else {
                                println!("wrapping from 1 to 5");
                                x3 = y2 - 100;
                                y3 = 1;
                                pending_direction = Some(1);
                            }
                        }
                        _ => {
                            if x2 <= 50 {
                                println!("wrapping from 2 to 4");
                                y3 = 50 + x2;
                                x3 = 51;
                                pending_direction = Some(0);
                            } else if x2 <= 100 {
                                println!("wrapping from 5 to 1");
                                y3 = 100 + x2;
                                x3 = 1;
                                pending_direction = Some(0);
                            } else {
                                println!("wrapping from 6 to 1");
                                x3 = x2 - 100;
                                y3 = 200;
                            }
                        }
                    }

                    x2 = x3;
                    y2 = y3;
                }

                match map.get(y2).and_then(|l| l.get(x2)) {
                    Some(&State::Wall) => {
                        println!("hit wall {x2} {y2} {direction} {pending_direction:?}");

                        break;
                    }
                    Some(&State::Empty) => {
                        x = x2;
                        y = y2;

                        if let Some(d) = pending_direction {
                            println!("turning to {d}");
                            direction = d;
                        }
                    }
                    _ => {
                        println!("{x} {y} {direction}");
                        panic!();
                    }
                }

                i -= 1;
            }
        } else {
            if part == "R" {
                println!("Turning right");
                direction += 1;
                direction %= 4;
            } else {
                println!("Turning left");

                if direction == 0 {
                    direction = 3;
                } else {
                    direction -= 1;
                }
            }

            println!("{x} {y} {direction}");
        }
    }

    println!("{}", y * 1000 + x * 4 + direction);
}
