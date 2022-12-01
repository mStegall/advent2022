pub fn main() {
    let data = include_str!("input.txt");

    let mut max = 0;
    let mut current = 0;

    for line in data.lines() {
        if let Ok(i) = line.parse::<i32>() {
            current += i
        } else {
            if current > max {
                max = current
            }
            current = 0
        }
    }

    if current > max {
        max = current
    }

    println!("{}", max)
}
