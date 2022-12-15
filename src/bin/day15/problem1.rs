use regex::Regex;

pub fn main() {
    let pattern =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let data = include_str!("./input.txt")
        .lines()
        .map(|l| {
            let c = pattern.captures(l).unwrap();
            let sx = c[1].parse::<isize>().unwrap();
            let sy = c[2].parse::<isize>().unwrap();
            let bx = c[3].parse::<isize>().unwrap();
            let by = c[4].parse::<isize>().unwrap();

            let d = (sx - bx).abs() + (sy - by).abs();

            ((sx, sy), (bx, by), d)
        })
        .collect::<Vec<_>>();

    let mut ranges = data
        .iter()
        .filter_map(|(s, _, d)| {
            let dy = (2000000 - s.1).abs();
            if dy < *d {
                return Some((s.0 - (d - dy), s.0 + (d - dy)));
            }

            return None;
        })
        .collect::<Vec<_>>();

    ranges.sort();

    let consolidated: Vec<(isize, isize)> = ranges.iter().fold(vec![], |mut acc, r| {
        if let Some(last) = acc.pop() {
            if last.1 >= r.0 {
                if r.1 > last.1 {
                    acc.push((last.0, r.1));
                } else {
                    acc.push((last.0, last.1));
                }
            } else {
                acc.push(last);
                acc.push(*r);
            }
        } else {
            acc.push(*r);
        }

        acc
    });
    println!("{}", consolidated[0].1 -consolidated[0].0)
}
