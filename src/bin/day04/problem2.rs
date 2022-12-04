use regex::Regex;

pub fn main() {
    let data = include_str!("./input.txt");

    let format = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    let result = data.lines().map(|l| {
        let captures = format.captures(l).unwrap();
        let l1 = captures[1].parse::<u32>().unwrap();
        let h1 = captures[2].parse::<u32>().unwrap();
        let l2 = captures[3].parse::<u32>().unwrap();
        let h2 = captures[4].parse::<u32>().unwrap();


        if l2 <= h1 && h2 >= l1 {
             return 1
        }

        if l1 <= h2 && h1 >= l2{
            return 1
       }
        0
    }).sum::<u32>();
    println!("{:?}", result)
}
