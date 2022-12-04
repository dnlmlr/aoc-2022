use std::{mem::transmute, simd::Simd};

use aoc_runner_derive::aoc;

/// Parse an ASCII string with either 1 or 2 characters to i64
#[inline(always)]
fn bstoi_2(num: &[u8]) -> i64 {
    if num.len() == 1 {
        (num[0] - b'0') as i64
    } else {
        num[0] as i64 * 10 + num[1] as i64 - (b'0' as i64 * 11)
    }
}

#[inline(always)]
fn parse_into_segments(mut dataset: &'_ [u8]) -> impl Iterator<Item = [i64; 4]> + '_ {
    if dataset.last() == Some(&b'\n') {
        dataset = &dataset[..dataset.len() - 1];
    }
    dataset.split(|&b| b == b'\n').map(|line| {
        let mut nums = [0; 4];

        line.split(|&b| b == b',')
            .flat_map(|half| half.split(|&b| b == b'-').map(bstoi_2))
            .enumerate()
            .for_each(|(i, num)| nums[i] = num as i64);
        nums
    })
}

#[aoc(day4, part1)]
pub fn day4_part1(dataset: &[u8]) -> i64 {
    parse_into_segments(dataset)
        .filter(|nums| {
            let [a, b, c, d] = nums;
            a >= c && b <= d || c >= a && d <= b
        })
        .count() as i64
}

#[aoc(day4, part2)]
pub fn day4_part2(dataset: &[u8]) -> i64 {
    parse_into_segments(dataset)
        .filter(|nums| {
            let [a, b, c, d] = nums;
            b >= c && d >= a
        })
        .count() as i64
}

macro_rules! extract_inc {
    ($dat:ident, $out:expr, $search:expr) => {
        if *$dat.add(1) == $search {
            $out = *$dat as i16;
            $dat = $dat.add(2);
        } else {
            $out = i16::from_be_bytes(*transmute::<*const u8, &[u8; 2]>($dat));
            $dat = $dat.add(3);
        }
    };
}

#[aoc(day4, part1, single_pass)]
pub fn day4_part1_optimized(dataset: &[u8]) -> i64 {
    let mut dat = dataset.as_ptr();
    let dat_end = dataset.as_ptr_range().end;

    let mut lhs = Simd::from_array([0, 0]);
    let mut rhs = Simd::from_array([0, 0]);

    let mut sum = 0;

    while dat < dat_end {
        unsafe {
            extract_inc!(dat, lhs[0], b'-');
            extract_inc!(dat, lhs[1], b',');
            extract_inc!(dat, rhs[0], b'-');
            extract_inc!(dat, rhs[1], b'\n');

            let diff = lhs - rhs;

            let is_subset =
                diff[0].is_negative() != diff[1].is_negative() || diff[0] == 0 || diff[1] == 0;
            if is_subset {
                sum += 1;
            }
        }
    }

    sum
}

#[aoc(day4, part2, single_pass)]
pub fn day4_part2_optimized(dataset: &[u8]) -> i64 {
    let mut dat = dataset.as_ptr();
    let dat_end = dataset.as_ptr_range().end;

    let mut lhs = Simd::from_array([0, 0]);
    let mut rhs = Simd::from_array([0, 0]);

    let mut sum = 0;

    while dat < dat_end {
        unsafe {
            extract_inc!(dat, lhs[0], b'-');
            extract_inc!(dat, rhs[1], b',');
            extract_inc!(dat, lhs[1], b'-');
            extract_inc!(dat, rhs[0], b'\n');

            let diff = lhs - rhs;

            let intersects = diff[0] <= 0 && diff[1] <= 0;
            if intersects {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = br#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_day4_part1() {
        assert_eq!(2, day4_part1(INPUT));
    }

    #[test]
    fn test_day4_part2() {
        assert_eq!(4, day4_part2(INPUT));
    }
}
