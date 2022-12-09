use std::collections::HashSet;

pub fn main() {
    let mut rope: Vec<(isize, isize)> = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    let mut tail_pos: HashSet<(isize, isize)> = HashSet::new();

    for line in include_str!("./input.txt").lines() {
        let (direction, distance_s) = line.split_once(' ').unwrap();
        let mut distance = distance_s.parse::<u32>().unwrap();

        while distance > 0 {
            match direction {
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                _ => (),
            }

            for i in 0..9 {
                let head = rope.get(i).unwrap().clone();
                let tail = rope.get_mut(i + 1).unwrap();
                if head.0 == tail.0 {
                    if (tail.1 - head.1) > 1 {
                        tail.1 -= 1;
                    } else if (head.1 - tail.1) > 1 {
                        tail.1 += 1;
                    }
                } else if head.1 == tail.1 {
                    if (tail.0 - head.0) > 1 {
                        tail.0 -= 1;
                    } else if (head.0 - tail.0) > 1 {
                        tail.0 += 1;
                    }
                } else if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                    if tail.0 > head.0 {
                        tail.0 -= 1;
                    } else if head.0 > tail.0 {
                        tail.0 += 1;
                    }

                    if tail.1 > head.1 {
                        tail.1 -= 1;
                    } else if head.1 > tail.1 {
                        tail.1 += 1;
                    }
                }
            }

            tail_pos.insert(rope[9]);

            distance -= 1;
        }
        println!("{rope:?}");
    }

    println!("{}", tail_pos.len())
}
