pub fn main() {
    let data = include_str!("./input.txt").chars().collect::<Vec<_>>();
    for i in 0..(data.len() - 3) {
        if data[i] != data[i + 1]
            && data[i] != data[i + 2]
            && data[i] != data[i + 3]
            && data[i + 1] != data[i + 2]
            && data[i + 1] != data[i + 3]
            && data[i + 2] != data[i + 3]
        {
            println!("{}", i + 4);
            return
        }
    }
}
