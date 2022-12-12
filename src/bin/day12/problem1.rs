use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn main() {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut map = include_str!("./input.txt")
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = (i, j);
                        (0, Some(0))
                    } else if c == 'E' {
                        end = (i, j);
                        ('z' as u32 - 'a' as u32, None)
                    } else {
                        (c as u32 - 'a' as u32, None)
                    }
                })
                .collect::<Vec<(u32, Option<u32>)>>()
        })
        .collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, start, 0)));

    while heap.len() > 0 {
        let Reverse((distance, (y, x), height)) = heap.pop().unwrap();

        for (dx, dy) in vec![(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
            let x2 = (dx + x as isize) as usize;
            let y2 = (dy + y as isize) as usize;
            if let Some(Some((h, None))) = map.get(y2).map(|l| l.get(x2)) {
                if *h <= height + 1 {
                    if (y2, x2) == end {
                        println!("{}", distance + 1);
                        return;
                    }

                    heap.push(Reverse((distance + 1, (y2, x2), *h)));

                    let field = map.get_mut(y2).unwrap().get_mut(x2).unwrap();
                    field.1 = Some(distance + 1);
                }
            }
        }
    }

    println!("{start:?} {end:?} {map:?}")
}
