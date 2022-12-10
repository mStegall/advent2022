pub fn main() {
    let mut x:isize = 1;
    let mut instructions = include_str!("./input.txt").lines();
    let mut signal_strength_sum = 0;

    let mut instruction_processing = "";
    let mut instruction_cycles_remaining = 0;

    let mut pixels:Vec<char> = vec!();

    for i in 0..240 {
        if instruction_cycles_remaining == 0 {
            let next = instructions.next().unwrap();
            if next.starts_with("addx") {
                instruction_cycles_remaining = 2;
            } else {
                instruction_cycles_remaining = 1;
            }
            instruction_processing = next;
        }

        let pixel = i % 40;
        if (x - pixel as isize).abs() <= 1 {
            pixels.push('#');
        } else {
            pixels.push('.');
        }

        if pixel == 39 {
            pixels.push('\n');
        }

        instruction_cycles_remaining -= 1;

        if instruction_cycles_remaining == 0 {
            if instruction_processing.starts_with("addx") {
                let modifier = instruction_processing
                    .strip_prefix("addx ")
                    .map(|s| s.parse::<isize>().unwrap())
                    .unwrap();

                x += modifier;
            }
        }
    }
    println!("{}", pixels.into_iter().collect::<String>())
}
