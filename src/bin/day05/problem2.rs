use regex::Regex;

pub fn main() {
    let data = include_str!("./input.txt");

    let split_data = data.split("\n\n").collect::<Vec<&str>>();

    let format = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let mut stacks: Vec<Vec<char>> = vec![];

    let mut stack_config_lines = split_data[0].lines().collect::<Vec<&str>>();
    stack_config_lines.reverse();
    let stack_count = (stack_config_lines[0].len() + 1) / 4;
    for _ in 0..stack_count {
        let stack = vec![];
        stacks.push(stack);
    }

    for line in stack_config_lines[1..].into_iter() {
        let chars = line.chars().collect::<Vec<_>>();
        for i in 0..stack_count {
            let c = chars[i * 4 + 1];
            if c != ' ' {
                stacks[i].push(c)
            }
        }
    }

    for line in split_data[1].lines() {
        let captures = format.captures(line).unwrap();
        let count = captures[1].parse::<usize>().unwrap();
        let src = captures[2].parse::<usize>().unwrap();
        let dest = captures[3].parse::<usize>().unwrap();

        let split_point = stacks[src - 1].len() - count;
        let mut i = stacks[src - 1].split_off(split_point);
        stacks[dest - 1].append(&mut i);
    }

    println!(
        "{}",
        String::from_iter(stacks.into_iter().filter_map(|mut s| s.pop()))
    )
}
