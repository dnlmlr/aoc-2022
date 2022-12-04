use aoc_runner_derive::aoc;

const ZERO4: u32 = b'0' as u32 + b'0' as u32 * 10 + b'0' as u32 * 100 + b'0' as u32 * 1_000;
const ZERO5: u32 = ZERO4 + b'0' as u32 * 10_000;
const MULS: [u32; 5] = [1, 10, 100, 1000, 10000];

#[aoc(day1, part1)]
pub fn day1_part1(dataset: &[u8]) -> i64 {
    let mut top1 = 0;
    let mut curr_sum = 0;

    let mut i = 0;
    let len = dataset.len();
    while i < len {
        let n = unsafe {
            [
                *dataset.get_unchecked(i + 4),
                *dataset.get_unchecked(i + 3),
                *dataset.get_unchecked(i + 2),
                *dataset.get_unchecked(i + 1),
                *dataset.get_unchecked(i),
            ]
        };

        curr_sum += if n[0] == b'\n' {
            i += 4;
            n[1] as u32 * MULS[0]
                + n[2] as u32 * MULS[1]
                + n[3] as u32 * MULS[2]
                + n[4] as u32 * MULS[3]
                - ZERO4
        } else {
            i += 5;
            n[0] as u32 * MULS[0]
                + n[1] as u32 * MULS[1]
                + n[2] as u32 * MULS[2]
                + n[3] as u32 * MULS[3]
                + n[4] as u32 * MULS[4]
                - ZERO5
        };

        if i >= len || unsafe { *dataset.get_unchecked(i + 1) } == b'\n' {
            top1 = top1.max(curr_sum);
            curr_sum = 0;
            i += 2;
        } else {
            i += 1;
        }
    }

    top1 as i64
}

#[aoc(day1, part2)]
pub fn day1_part2(dataset: &[u8]) -> i64 {
    let mut top3 = [0, 0, 0, 0];

    let mut i = 0;
    let len = dataset.len();
    while i < len {
        let n = unsafe {
            [
                *dataset.get_unchecked(i + 4),
                *dataset.get_unchecked(i + 3),
                *dataset.get_unchecked(i + 2),
                *dataset.get_unchecked(i + 1),
                *dataset.get_unchecked(i),
            ]
        };

        i += 4;

        top3[0] += if i >= len || n[0] == b'\n' {
            n[1] as u32 * MULS[0]
                + n[2] as u32 * MULS[1]
                + n[3] as u32 * MULS[2]
                + n[4] as u32 * MULS[3]
                - ZERO4
        } else {
            i += 1;
            n[0] as u32 * MULS[0]
                + n[1] as u32 * MULS[1]
                + n[2] as u32 * MULS[2]
                + n[3] as u32 * MULS[3]
                + n[4] as u32 * MULS[4]
                - ZERO5
        };

        if i + 1 >= len || unsafe { *dataset.get_unchecked(i + 1) } == b'\n' {
            if top3[0] >= top3[3] {
                top3 = [0, top3[2], top3[3], top3[0]];
            } else if top3[0] >= top3[2] {
                top3 = [0, top3[2], top3[0], top3[3]];
            } else if top3[0] >= top3[1] {
                top3 = [0, top3[0], top3[2], top3[3]]
            } else {
                top3[0] = 0;
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    top3.into_iter().skip(1).sum::<u32>() as i64
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = br#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_day1_part1() {
        assert_eq!(24000, day1_part1(INPUT));
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(45000, day1_part2(INPUT));
    }
}
