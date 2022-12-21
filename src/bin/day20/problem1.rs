pub fn main() {
    let mut data = include_str!("./input.txt")
        .lines()
        .filter_map(|s| s.parse::<isize>().ok())
        .map(|i| (i, false))
        .collect::<Vec<_>>();

    let mut remaining = data.len();
    let mut i = 0;

    while remaining > 0 {
        if let Some((_, false)) = data.get(i) {
            remaining -= 1;
            let (j, _) = data.remove(i);
            let mut new_index = (i as isize + j) % data.len() as isize;
            if new_index < 0 {
                new_index += data.len() as isize;
            }
            // println!("moving {j} from {i} to {new_index}");
            data.insert(new_index.try_into().unwrap(), (j, true));
        } else {
            i += 1;
            i %= data.len();
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
