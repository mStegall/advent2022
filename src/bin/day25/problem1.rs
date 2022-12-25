pub fn main() {
    let data = dec_to_snafu(
        include_str!("input.txt")
            .lines()
            .map(|s| snafu_to_dec(s))
            .sum(),
    );

    println!("{data:?}");

}

fn snafu_to_dec(s: &str) -> isize {
    let mut place = 1;

    let mut result = 0;

    for c in s.chars().rev() {
        if c == '1' {
            result += place;
        } else if c == '2' {
            result += place * 2;
        } else if c == '-' {
            result -= place;
        } else if c == '=' {
            result -= place * 2;
        }

        place *= 5;
    }

    return result;
}

fn dec_to_snafu(mut i: isize) -> String {
    let mut result = String::new();
    let mut place = 1;

    while i != 0  {
        let next = place * 5;
        let l = i % next;
        let r = l / place;
        if r < 3 {
            i -= l;
            result.push_str(&r.to_string())
        } else if r == 3 {
            i += place * 2;
            result.push('=');
        } else {
            i += place;
            result.push('-');
        }
        place *= 5;
    }


    return result.chars().rev().collect();
}
