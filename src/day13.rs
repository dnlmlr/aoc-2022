use aoc_runner_derive::aoc;

struct StreamParser<'a> {
    lhs: &'a [u8],
    rhs: &'a [u8],

    lhs_extra_brackets: u8,
    rhs_extra_brackets: u8,
}

impl<'a> StreamParser<'a> {
    fn new(lhs: &'a [u8], rhs: &'a [u8]) -> Self {
        Self {
            lhs,
            rhs,
            lhs_extra_brackets: 0,
            rhs_extra_brackets: 0,
        }
    }

    fn identify(ch: u8) -> u8 {
        if ch.is_ascii_digit() {
            b'0'
        } else {
            ch
        }
    }

    fn extract_val(stream: &mut &[u8]) -> u8 {
        let val;
        if stream[0].is_ascii_digit() {
            if stream[1].is_ascii_digit() {
                val = (stream[0] & 0xf) * 10 + (stream[1] & 0xf);
                *stream = &stream[2..];
            } else {
                val = stream[0] & 0xf;
                *stream = &stream[1..];
            }
        } else {
            val = stream[0];
            *stream = &stream[1..];
        }
        val
    }

    fn extract_val_lhs(&mut self) -> u8 {
        if Self::identify(self.lhs[0]) != b'0' && self.lhs_extra_brackets > 0 {
            self.lhs_extra_brackets -= 1;
            b']'
        } else {
            Self::extract_val(&mut self.lhs)
        }
    }

    fn extract_val_rhs(&mut self) -> u8 {
        if Self::identify(self.rhs[0]) != b'0' && self.rhs_extra_brackets > 0 {
            self.rhs_extra_brackets -= 1;
            b']'
        } else {
            Self::extract_val(&mut self.rhs)
        }
    }

    fn compare(&mut self) -> i8 {
        loop {
            let type_lhs = Self::identify(self.lhs[0]);
            let type_rhs = Self::identify(self.rhs[0]);

            if type_lhs == b'[' && type_rhs == b'0' {
                self.lhs = &self.lhs[1..];
                self.rhs_extra_brackets += 1;
                continue;
            }

            if type_lhs == b'0' && type_rhs == b'[' {
                self.rhs = &self.rhs[1..];
                self.lhs_extra_brackets += 1;
                continue;
            }

            let diff = if type_lhs == b'0' && type_rhs == b'0' {
                self.extract_val_rhs() as i8 - self.extract_val_lhs() as i8
            } else {
                self.extract_val_lhs() as i8 - self.extract_val_rhs() as i8
            };

            if diff != 0 {
                return diff;
            }
        }
    }
}

#[aoc(day13, part1)]
pub fn day13_part1(dataset: &[u8]) -> i64 {
    let mut lines = dataset.split(|&b| b == b'\n');

    let mut idx = 0;
    let mut sum = 0;
    while let (Some(lhs), Some(rhs)) = (lines.next(), lines.next()) {
        lines.next();

        idx += 1;
        if StreamParser::new(lhs, rhs).compare() > 0 {
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
