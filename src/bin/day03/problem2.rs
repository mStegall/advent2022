use itertools::Itertools;

pub fn main() {
    let data = include_str!("./input.txt");

    let result = data.lines().tuples().map(|(e1,e2,e3)| {
        for c in e1.chars() {
            if e2.contains(c) && e3.contains(c){
                if c.is_lowercase() {
                    return c as u32 - 96
                }
                return c as u32 - 64 + 26


            }
        }
        0
    }).sum::<u32>();

    println!("{:?}", result)
}
