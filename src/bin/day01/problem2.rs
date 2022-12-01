pub fn main() {
    let data = include_str!("input.txt");

    let mut current = 0;

    let mut elfs:Vec<i32> = vec!();

    for line in data.lines() {
        if let Ok(i) = line.parse::<i32>() {
            current += i
        } else {
            elfs.push(current);
            current = 0
        }
    }

    if current > 0 {
        elfs.push(current);
    }
    elfs.sort();
    elfs.reverse();
    println!("{:?}", elfs.as_slice()[0..3].into_iter().sum::<i32>())
}
