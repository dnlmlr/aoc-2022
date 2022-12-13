use std::cmp::Ordering;

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Value(u8),
}

impl Packet {
    fn parse_list(data: &[u8]) -> (Self, usize) {
        let mut packets = Vec::new();
        let mut i = 0;
        while data[i] != b']' {
            let val = match data[i] {
                b'[' => {
                    let (val, ii) = Self::parse_list(&data[i + 1..]);
                    i += ii;
                    val
                }
                mut c => {
                    c = c & 0xf;
                    if data[i + 1].is_ascii_digit() {
                        c = c * 10 + (data[i + 1] & 0xf);
                        i += 1;
                    }

                    if data[i + 1] == b',' {
                        i += 1;
                    }
                    Self::Value(c)
                }
            };
            packets.push(val);
            i += 1;
        }
        i += 1;
        if data.get(i) == Some(&b',') {
            i += 1;
        }

        (Self::List(packets), i)
    }

    fn correct_order(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Value(lhs), Self::Value(rhs)) => lhs.cmp(rhs),
            (Self::Value(_), Self::List(_)) => Self::List(vec![self.clone()]).correct_order(other),
            (Self::List(_), Self::Value(_)) => self.correct_order(&Self::List(vec![other.clone()])),
            (Self::List(lhs), Self::List(rhs)) => {
                for i in 0..lhs.len().min(rhs.len()) {
                    let ord = lhs[i].correct_order(&rhs[i]);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                if lhs.len() != rhs.len() {
                    if lhs.len() < rhs.len() {
                        return Ordering::Less
                    } else {
                        return  Ordering::Greater;
                    }
                } else {
                    Ordering::Equal
                }
            }
        }
    }

    // fn serialize(&self) -> String {
    //     match self {
    //         Packet::List(paks) => {
    //             let mut s = "[".to_string();
    //             for i in 0..paks.len() {
    //                 let p = &paks[i];
    //                 s.push_str(&p.serialize());
    //                 if i < paks.len() - 1 {
    //                     s.push(',');
    //                 }
    //             }
    //             s.push(']');
    //             s
    //         }
    //         Packet::Value(c) => format!("{c}"),
    //     }
    // }
}

#[aoc(day13, part1)]
pub fn day13_part1(dataset: &[u8]) -> i64 {
    let mut lines = dataset.split(|&b| b == b'\n');

    let mut idx = 0;
    let mut sum = 0;
    while let (Some(lhs), Some(rhs)) = (lines.next(), lines.next()) {
        lines.next();

        let packet_lhs = Packet::parse_list(&lhs[1..]).0;
        let packet_rhs = Packet::parse_list(&rhs[1..]).0;

        // let s_lhs = packet_lhs.serialize();
        // let s_rhs = packet_rhs.serialize();

        // assert_eq!(String::from_utf8_lossy(lhs), s_lhs);
        // assert_eq!(String::from_utf8_lossy(rhs), s_rhs);

        idx += 1;
        if packet_lhs.correct_order(&packet_rhs) == Ordering::Less {
            sum += idx;
        }
    }

    sum
}

// #[aoc(day13, part2)]
// pub fn day13_part2(_dataset: &[u8]) -> i64 {
//     todo!()
// }

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = br#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

    #[test]
    fn test_day13_part1() {
        assert_eq!(13, day13_part1(INPUT));
    }

    // #[test]
    // fn test_day13_part2() {
    //     assert_eq!(45000, day13_part2(INPUT));
    // }
}
