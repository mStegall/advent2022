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

    if round[1] == "X" {
        score += 1
    } else if round[1] == "Y" {
        score += 2
    } else {
        score += 3
    }

    if (round[1] == "X" && round[0] == "A")
        || (round[1] == "Y" && round[0] == "B")
        || (round[1] == "Z" && round[0] == "C")
    {
        score += 3
    } else if (round[1] == "X" && round[0] == "C")
        || (round[1] == "Y" && round[0] == "A")
        || (round[1] == "Z" && round[0] == "B")
    {
        score += 6
    }

    score
}
