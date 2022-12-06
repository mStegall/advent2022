use std::collections::HashMap;

pub fn main() {
    let data = include_str!("./input.txt").chars().collect::<Vec<_>>();

    let mut map = HashMap::new();

    for i in 0..14 {
        map.entry(data[i]).and_modify(|i| *i += 1).or_insert(1);
    }

    for i in 14..data.len() {
        map.entry(data[i]).and_modify(|i| *i += 1).or_insert(1);

        let old_char_count = map.get(&data[i - 14]).unwrap() - 1;
        if old_char_count == 0 {
            map.remove(&data[i - 14]);
        } else {
            map.entry(data[i - 14]).and_modify(|i| *i = old_char_count);
        }

        if map.len() == 14 {
            println!("{}", i + 1);
            return;
        }
    }

    println!("{:?}", map);
}
