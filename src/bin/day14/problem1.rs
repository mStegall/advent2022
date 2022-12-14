use std::fmt;

#[derive(PartialEq)]
enum State {
    Rock,
    Air,
    Sand,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Rock => f.write_str("#"),
            State::Air => f.write_str("."),
            State::Sand => f.write_str("o"),
        }
    }
}

pub fn main() {
    let instructions = include_str!("./input.txt")
        .lines()
        .map(|l| {
            l.split(" -> ")
                .filter_map(|p| p.split_once(','))
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let left = instructions
        .iter()
        .flat_map(|i| i.iter().map(|(x, _)| x))
        .min()
        .unwrap();
    let right = instructions
        .iter()
        .flat_map(|i| i.iter().map(|(x, _)| x))
        .max()
        .unwrap();
    let deep = instructions
        .iter()
        .flat_map(|i| i.iter().map(|(_, y)| y))
        .max()
        .unwrap();

    let mut field = (0..(*deep + 1))
        .map(|_| {
            (*left..(*right + 1))
                .map(|_| State::Air)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for i in instructions.iter() {
        let (mut x, mut y) = i.first().unwrap();
        let mut t = field.get_mut(y).unwrap().get_mut(x - left).unwrap();
        *t = State::Rock;

        for (ddx, ddy) in i[1..].iter() {
            let dx = *ddx;
            let dy = *ddy;
            while dx != x || dy != y {
                if dx < x {
                    x -= 1;
                } else if dx > x {
                    x += 1;
                } else if dy > y {
                    y += 1;
                } else if dy < y {
                    y -= 1;
                }

                let mut t = field.get_mut(y).unwrap().get_mut(x - left).unwrap();
                *t = State::Rock;
            }
        }
    }

    let mut sand_count = 0;


    'outer: loop {
        let mut x = 500 - left;
        let mut y = 0;

        loop {
            if y +1 >= field.len() {
                break 'outer;
            }
            if field[y + 1][x] == State::Air {
                y += 1;
            } else if x == 0 {
                break 'outer;
            } else if field[y + 1][x - 1] == State::Air {
                y += 1;
                x -= 1;
            } else if x >= right - left {
                break 'outer;
            } else if field[y + 1][x + 1] == State::Air {
                y += 1;
                x += 1;
            } else {
                let mut t = field.get_mut(y).unwrap().get_mut(x).unwrap();
                *t = State::Sand;
                sand_count += 1;
                break;
            }
        }
    }

    println!(
        "{}\n{sand_count}",
        field
            .iter()
            .map(|l| l.iter().map(|i| format!("{i:?}")).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    )
}
