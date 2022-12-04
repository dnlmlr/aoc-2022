use aoc_runner_derive::aoc;

#[inline(always)]
fn index_letter(letter: u8) -> u8 {
    letter & 0b00_111111
}

#[aoc(day3, part1)]
pub fn day3_part1(dataset: &[u8]) -> i64 {
    dataset
        .split(|b| *b == b'\n')
        .map(|line| {
            let compartment_len = line.len() / 2;
            let mut bitflags = [0_u64; 2];

            line[..compartment_len]
                .iter()
                .copied()
                .map(index_letter)
                .for_each(|idx| {
                    bitflags[0] |= 1 << idx;
                });

            line[compartment_len..]
                .iter()
                .copied()
                .map(index_letter)
                .for_each(|idx| {
                    bitflags[1] |= 1 << idx;
                });

            let duplicate = (bitflags[0] & bitflags[1]).trailing_zeros();

            if duplicate <= 26 {
                duplicate as i64 + 26
            } else {
                duplicate as i64 - (b'a' - b'A') as i64
            }
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn day3_part2(dataset: &[u8]) -> i64 {
    dataset
        .split(|b| *b == b'\n')
        .array_chunks::<3>()
        .map(|lines| {
            let mut bitflags = [0_u64; 3];
            lines.iter().enumerate().for_each(|(i, line)| {
                line.iter().copied().map(index_letter).for_each(|idx| {
                    bitflags[i] |= 1 << idx;
                });
            });

            let duplicate = (bitflags[0] & bitflags[1] & bitflags[2]).trailing_zeros();

            if duplicate <= 26 {
                duplicate as i64 + 26
            } else {
                duplicate as i64 - (b'a' - b'A') as i64
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = br#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_day3_part1() {
        assert_eq!(157, day3_part1(INPUT));
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!(70, day3_part2(INPUT));
    }
}
