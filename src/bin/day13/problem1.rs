#[derive(Debug, PartialEq)]
enum SignalData {
    List(Vec<SignalData>),
    Data(u32),
}

impl SignalData {
    fn lt(&self, other: &SignalData) -> bool {
        if let SignalData::Data(i) = self {
            if let SignalData::Data(j) = other {
                return i < j;
            } else {
                return SignalData::List(vec![SignalData::Data(*i)]).lt(other);
            }
        }

        if let SignalData::List(is) = self {
            if let SignalData::List(js) = other {
                for (x, i) in is.iter().enumerate() {
                    if let Some(j) = js.get(x) {
                        if i == j {
                            continue;
                        } else if i.lt(j) {
                            return true;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                if js.len() == is.len() {
                    return false;
                } else {
                    return true;
                }
            } else if let SignalData::Data(j) = other {
                return self.lt(&SignalData::List(vec![SignalData::Data(*j)]));
            }
        }

        false
    }
}

pub fn main() {
    let data = include_str!("./input.txt")
        .split("\n\n")
        .map(|c| c.lines().map(|s| parse_signal(s)).collect::<Vec<_>>())
        .enumerate()
        .map(|(i, l)| if l[0].lt(&l[1]) { i + 1 } else { 0 })
        .sum::<usize>();

    println!("{data:?}")

    // let s = SignalData::List(vec![SignalData::Data(9)]);

    // println!(
    //     "{}",
    //     s.lt(&SignalData::List(vec!(SignalData::List(vec!(
    //         SignalData::Data(8),
    //         SignalData::Data(7)
    //     )))))
    // )
}

fn parse_signal(s: &str) -> SignalData {
    parse_signal_inner(s).0
}

fn parse_signal_inner<'a>(s: &'a str) -> (SignalData, &'a str) {
    if s.starts_with("[") {
        let mut v: Vec<SignalData> = vec![];
        let mut d = s.strip_prefix("[").unwrap();

        while d.len() > 0 {
            if d.starts_with("]") {
                return (SignalData::List(v), d.trim_start_matches(&[']', ',']));
            }

            let res = parse_signal_inner(d);
            v.push(res.0);
            d = res.1;
        }

        return (SignalData::List(v), d);
    } else {
        let first_comma = s.find(',');
        let first_bracket = s.find(']');
        if let Some(n) = first_bracket {
            if let Some(m) = first_comma {
                if n < m {
                    let (d, remaining) = s.split_at(n);
                    return (SignalData::Data(d.parse::<u32>().unwrap()), remaining);
                }
            }
        }
        if let Some((d, remaining)) = s.split_once(',') {
            return (SignalData::Data(d.parse::<u32>().unwrap()), remaining);
        } else {
            return (
                SignalData::Data(s.trim_end_matches(']').parse::<u32>().unwrap()),
                "",
            );
        }
    }
}
