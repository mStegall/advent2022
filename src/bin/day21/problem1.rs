use std::collections::HashMap;

pub fn main() {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in include_str!("./test.txt").lines() {
        let (name, ops_str) = l.split_once(": ").unwrap();
        let ops = ops_str.split(" ").collect::<Vec<_>>();
        map.insert(name, ops);
    }

    println!("{:?}", get_value("root", &map));
}

fn get_value(n: &str, map: &HashMap<&str, Vec<&str>>) -> usize {
    let ops = map.get(n).unwrap();
    if ops.len() == 1 {
        return ops[0].parse().unwrap();
    }
    let left = get_value(ops[0], &map);
    let right = get_value(ops[2], &map);

    match ops[1] {
        "*" => left * right,
        "/" => left / right,
        "-" => left - right,
        _ => left + right,
    }
}
