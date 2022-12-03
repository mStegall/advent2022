pub fn main() {
    let data = include_str!("./input.txt");

    println!(
        "{:?}",
        data.lines()
            .map(|l| score(l.split(' ').collect()))
            .sum::<i32>()
    )
}

fn score(round: Vec<&str>) -> i32 {
    let mut score = 0;

    if round[1] == "Y" {
        score += 3
    } else if round[1] == "Z" {
        score += 6
    }

    score += match (round[1],round[0]) {
        ("Y", "A") => 1,
        ("Y", "B") => 2,
        ("Y", "C") => 3,
        ("Z", "A") => 2,
        ("Z", "B") => 3,
        ("Z", "C") => 1,
        ("X", "A") => 3,
        ("X", "B") => 1,
        ("X", "C") => 2,
        (_,_)=>0
    };

    score
}
