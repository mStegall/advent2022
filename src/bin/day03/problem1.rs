pub fn main() {
    let data = include_str!("./input.txt");

    let result = data.lines().map(|l| {
        let len = l.len();
        let half1 = &l[0..len/2];
        let half2 = &l[len/2..];
        for c in half1.chars() {
            if half2.contains(c){
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
