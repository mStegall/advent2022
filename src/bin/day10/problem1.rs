pub fn main() {
    let mut x = 1;
    let mut instructions = include_str!("./input.txt").lines();
    let mut signal_strength_sum = 0;

    let mut instruction_processing = "";
    let mut instruction_cycles_remaining = 0;

    for i in 1..221 {
        // println!("cycle {i}");
        
        if instruction_cycles_remaining == 0 {
            let next = instructions.next().unwrap();
            if next.starts_with("addx") {
                instruction_cycles_remaining = 2;
            } else {
                instruction_cycles_remaining = 1;
            }
            instruction_processing = next;
            // println!("Read next instruction {instruction_processing} will take {instruction_cycles_remaining} cycles")
        }

        if ((i as isize - 20) % 40) == 0 {
            println!("adding signal strength {}",x * i);
            signal_strength_sum += x * i
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
    println!("{signal_strength_sum}")
}
