use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum SignalData {
    List(Vec<SignalData>),
    Data(u32),
}

impl Ord for SignalData {
    fn cmp(&self, other: &SignalData) -> Ordering {
        match self {
            SignalData::Data(i) => {
                if let SignalData::Data(j) = other {
                    return i.cmp(j);
                } else {
                    return SignalData::List(vec![SignalData::Data(*i)]).cmp(other);
                }
            }
            SignalData::List(is) => match other {
                SignalData::List(js) => {
                    for (x, i) in is.iter().enumerate() {
                        if let Some(j) = js.get(x) {
                            if i == j {
                                continue;
                            } else if i.lt(j) {
                                return Ordering::Less;
                            } else {
                                return Ordering::Greater;
                            }
                        } else {
                            return Ordering::Greater;
                        }
                    }
                    if js.len() == is.len() {
                        return Ordering::Equal;
                    } else {
                        return Ordering::Less;
                    }
                }
                SignalData::Data(j) => {
                    return self.cmp(&SignalData::List(vec![SignalData::Data(*j)]));
                }
            },
        }
    }
}

impl PartialOrd for SignalData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn main() {
    let mut data = include_str!("./input.txt")
        .lines()
        .filter(|l| *l != "")
        .map(|s| parse_signal(s))
        .collect::<Vec<_>>();

    data.push(SignalData::List(vec![SignalData::List(vec![
        SignalData::Data(2),
    ])]));
    data.push(SignalData::List(vec![SignalData::List(vec![
        SignalData::Data(6),
    ])]));

    data.sort();

    let i1 = data
        .iter()
        .position(|i| *i == SignalData::List(vec![SignalData::List(vec![SignalData::Data(2)])]));
    let i2 = data
        .iter()
        .position(|i| *i == SignalData::List(vec![SignalData::List(vec![SignalData::Data(6)])]));

    println!("{}", (i1.unwrap() + 1) * (i2.unwrap() + 1))
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
