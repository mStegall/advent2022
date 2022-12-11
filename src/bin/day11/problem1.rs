#[derive(Debug)]
struct Monkey {
    current_items: Vec<usize>,
    mult: bool,
    factor: Option<usize>,
    test: usize,
    next_true: usize,
    next_false: usize,
    inspected: usize,
}

pub fn main() {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut recieving: Vec<Vec<usize>> = vec![];

    for monkey_data in include_str!("./input.txt").split("\n\n") {
        let lines = monkey_data.lines().collect::<Vec<_>>();
        let monkey_items = lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<_>>();

        let monkey_op = lines[2].strip_prefix("  Operation: new = old ").unwrap();
        let (mult, factor) = if monkey_op == "* old" {
            (true, None)
        } else if monkey_op.starts_with("+") {
            let n = monkey_op.split(' ').last().unwrap().parse::<usize>().unwrap();
            (false, Some(n))
        } else {
            let n = monkey_op.split(' ').last().unwrap().parse::<usize>().unwrap();
            (true, Some(n))
        };
        let test = lines[3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let next_true = lines[4].chars().last().unwrap().to_digit(10).unwrap() as usize;
        let next_false = lines[5].chars().last().unwrap().to_digit(10).unwrap() as usize;
        let monkey = Monkey {
            current_items: monkey_items,
            mult,
            factor,
            test,
            next_true,
            next_false,
            inspected: 0,
        };
        monkeys.push(monkey);
        recieving.push(vec![]);
    }

    for _ in 0..20 {
        for (i, mut monkey) in &mut monkeys.iter_mut().enumerate() {
            monkey.current_items.append(recieving.get_mut(i).unwrap());

            for item in monkey.current_items.drain(..) {
                monkey.inspected += 1;

                let mut modified = item;
                if monkey.mult {
                    if let Some(n) = monkey.factor {
                        modified *= n;
                    } else {
                        modified *= modified;
                    }
                } else {
                    if let Some(n) = monkey.factor {
                        modified += n;
                    }
                }

                modified /= 3;

                if modified % monkey.test == 0 {
                    recieving.get_mut(monkey.next_true).unwrap().push(modified)
                } else {
                    recieving.get_mut(monkey.next_false).unwrap().push(modified)
                }

            }
        }
    }

    let mut insepcted = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    insepcted.sort();
    insepcted.reverse();
    println!("{:?}", insepcted[0] * insepcted[1])
}
