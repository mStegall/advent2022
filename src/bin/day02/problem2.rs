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

    if round[1] == "Y" {
        if round[0] == "A" {
            score += 1
        } else if round[0] == "B" {
            score += 2
        } else if round[0] == "C" {
            score += 3
        }
    } else if round[1] == "Z" {
        if round[0] == "A" {
            score += 2
        } else if round[0] == "B" {
            score += 3
        } else if round[0] == "C" {
            score += 1
        }
    } else if round[1] == "X" {
        if round[0] == "A" {
            score += 3
        } else if round[0] == "B" {
            score += 1
        } else if round[0] == "C" {
            score += 2
        }
    }

    score
}
