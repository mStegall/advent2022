use std::collections::HashMap;

pub fn main() {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in include_str!("./input.txt").lines() {
        let (name, ops_str) = l.split_once(": ").unwrap();
        let ops = ops_str.split(" ").collect::<Vec<_>>();
        map.insert(name, ops);
    }

    let root = map.get("root").unwrap();

    println!("{:?}", get_human_value("root", 0, &map));
}

fn get_human_value(n: &str, target: usize, map: &HashMap<&str, Vec<&str>>) -> usize {
    if n == "humn" {
        return target;
    }
    let ops = map.get(n).unwrap();

    if has_human(ops[0], map) {
        let other_value = get_value(ops[2], map);
        if n == "root" {
            return get_human_value(ops[0], other_value, map);
        }

        let target2 = match ops[1] {
            "*" => target / other_value,
            "/" => target * other_value,
            "-" => target + other_value,
            _ => target - other_value,
        };

        return get_human_value(ops[0], target2, map);
    } else {
        let other_value = get_value(ops[0], map);
        if n == "root" {
            return get_human_value(ops[2], other_value, map);
        }

        let target2 = match ops[1] {
            "*" => target / other_value,
            "/" => other_value / target,
            "-" => other_value - target,
            _ => target - other_value,
        };

        return get_human_value(ops[2], target2, map);
    }
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

fn has_human(n: &str, map: &HashMap<&str, Vec<&str>>) -> bool {
    if n == "humn" {
        return true;
    }
    let ops = map.get(n).unwrap();
    if ops.len() == 1 {
        return false;
    }
    let left = has_human(ops[0], &map);
    let right = has_human(ops[2], &map);

    return left || right;
}
