pub fn main() {
    let mut data = include_str!("./input.txt")
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .enumerate()
        .map(|(j, i)| (i * 811589153, j))
        .collect::<Vec<_>>();

    for _ in 0..10 {
        let mut i = 0;
        while i < data.len() {
            let t = data.iter().position(|(_, s)| *s == i).unwrap();
            let e = data.remove(t);
            let mut new_index = (t as isize + e.0) % data.len() as isize;
            if new_index < 0 {
                new_index += data.len() as isize;
            }
            // println!("moving {j} from {i} to {new_index}");
            data.insert(new_index.try_into().unwrap(), e);

            i += 1;
        }

        // println!("{data:?}");
    }

    let base = data.iter().position(|(s, _)| *s == 0).unwrap();

    let values: isize = vec![1000, 2000, 3000]
        .iter()
        .map(|i| data[(i + base) % data.len()].0)
        .sum();

    println!("{values:?}");
}
