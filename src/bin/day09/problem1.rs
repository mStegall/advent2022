use std::collections::HashSet;

pub fn main() {
    let mut head_x: isize = 0;
    let mut head_y: isize = 0;
    let mut tail_x: isize = 0;
    let mut tail_y: isize = 0;

    let mut tail_pos: HashSet<(isize, isize)> = HashSet::new();

    for line in include_str!("./input.txt").lines() {
        let (direction, distance_s) = line.split_once(' ').unwrap();
        let mut distance = distance_s.parse::<u32>().unwrap();

        while distance > 0 {
            match direction {
                "R" => head_x += 1,
                "L" => head_x -= 1,
                "U" => head_y += 1,
                "D" => head_y -= 1,
                _ => (),
            }

            if head_x == tail_x {
                if (tail_y - head_y) > 1 {
                    tail_y -= 1;
                } else if (head_y - tail_y) > 1 {
                    tail_y += 1;
                }
            } else if head_y == tail_y {
                if (tail_x - head_x) > 1 {
                    tail_x -= 1;
                } else if (head_x - tail_x) > 1 {
                    tail_x += 1;
                }
            } else if (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1 {
                if tail_x > head_x {
                    tail_x -= 1;
                } else if head_x > tail_x {
                    tail_x += 1;
                }

                if tail_y > head_y {
                    tail_y -= 1;
                } else if head_y > tail_y {
                    tail_y += 1;
                }
            }

            tail_pos.insert((tail_x, tail_y));

            distance -= 1;
        }
    }

    println!("{}", tail_pos.len())
}
