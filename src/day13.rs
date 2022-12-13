use core::slice::memchr::memchr;

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

    #[inline(always)]
    fn extract_val(stream: &mut &[u8], is_digit: bool) -> u8 {
        let val;
        if is_digit {
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

    #[inline(always)]
    fn extract_val_lhs(&mut self, is_digit: bool) -> u8 {
        if self.lhs_extra_brackets == 0 || is_digit {
            Self::extract_val(&mut self.lhs, is_digit)
        } else {
            self.lhs_extra_brackets -= 1;
            b']'
        }
    }

    #[inline(always)]
    fn extract_val_rhs(&mut self, is_digit: bool) -> u8 {
        if self.rhs_extra_brackets == 0 || is_digit {
            Self::extract_val(&mut self.rhs, is_digit)
        } else {
            self.rhs_extra_brackets -= 1;
            b']'
        }
    }

    fn compare(&mut self) -> i8 {
        loop {
            let digit_lhs = self.lhs[0].is_ascii_digit();
            let digit_rhs = self.rhs[0].is_ascii_digit();

            if self.lhs[0] == b'[' && digit_rhs {
                self.lhs = &self.lhs[1..];
                self.rhs_extra_brackets += 1;
                continue;
            }

            if digit_lhs && self.rhs[0] == b'[' {
                self.rhs = &self.rhs[1..];
                self.lhs_extra_brackets += 1;
                continue;
            }

            let diff = if digit_lhs && digit_rhs {
                self.extract_val_rhs(digit_rhs) as i8 - self.extract_val_lhs(digit_lhs) as i8
            } else {
                self.extract_val_lhs(digit_lhs) as i8 - self.extract_val_rhs(digit_rhs) as i8
            };

            if diff != 0 {
                return diff;
            }
        }
    }
}

#[aoc(day13, part1)]
pub fn day13_part1(dataset: &[u8]) -> i64 {
    let mut idx = 0;
    let mut sum = 0;

    let mut i = 0;
    while i < dataset.len() {
        let Some(lhs_len) = memchr(b'\n', &dataset[i..]) else { break; };
        let lhs = &dataset[i..i + lhs_len];
        i += lhs_len + 1;

        let Some(rhs_len) = memchr(b'\n', &dataset[i..]) else { break; };
        let rhs = &dataset[i..i + rhs_len];
        i += rhs_len + 2;

        idx += 1;
        if StreamParser::new(lhs, rhs).compare() > 0 {
            sum += idx;
        }
    }

    sum
}

#[aoc(day13, part2)]
pub fn day13_part2(dataset: &[u8]) -> i64 {
    let (marker1, mut pos1) = (b"[[2]]", 1);
    let (marker2, mut pos2) = (b"[[6]]", 2);

    let mut i = 0;
    while i < dataset.len() {
        let Some(lhs_len) = memchr(b'\n', &dataset[i..]) else { break; };
        let lhs = &dataset[i..i + lhs_len];
        i += lhs_len + 1;

        let Some(rhs_len) = memchr(b'\n', &dataset[i..]) else { break; };
        let rhs = &dataset[i..i + rhs_len];
        i += rhs_len + 2;

        if StreamParser::new(lhs, marker2).compare() > 0 {
            if StreamParser::new(lhs, marker1).compare() > 0 {
                pos1 += 1;
            }
            pos2 += 1;
        }

        if StreamParser::new(rhs, marker2).compare() > 0 {
            if StreamParser::new(rhs, marker1).compare() > 0 {
                pos1 += 1;
            }
            pos2 += 1;
        }
    }

    pos1 * pos2
}

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

    #[test]
    fn test_day13_part2() {
        assert_eq!(140, day13_part2(INPUT));
    }
}
