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
fn parse_into_segments(dataset: &'_ [u8]) -> impl Iterator<Item = [i64; 4]> + '_ {
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
pub fn day4_part1(mut dataset: &[u8]) -> i64 {
    if dataset.last() == Some(&b'\n') {
        dataset = &dataset[..dataset.len() - 1];
    }
    parse_into_segments(dataset)
        .filter(|nums| {
            nums[0] >= nums[2] && nums[1] <= nums[3] || nums[2] >= nums[0] && nums[3] <= nums[1]
        })
        .count() as i64
}

#[aoc(day4, part2)]
pub fn day4_part2(dataset: &[u8]) -> i64 {
    parse_into_segments(dataset)
        .filter(|nums| {
            nums[0] >= nums[2] && nums[0] <= nums[3]
                || nums[1] >= nums[2] && nums[1] <= nums[3]
                || nums[2] >= nums[0] && nums[2] <= nums[1]
                || nums[3] >= nums[0] && nums[3] <= nums[1]
        })
        .count() as i64
}

#[aoc(day4, part1, single_pass)]
pub fn day4_part1_optimized(dataset: &[u8]) -> i64 {
    // The bits that are set in the mask uniquely identifiy if the character is a number,
    // newline or either '-' / ','.
    // - For numbers both bits are set
    // - For '-' and ',' only the first bit is set
    // - For newline none of the bits are set
    const TYPE_MASK: u8 = 0b00110000;
    const DIGIT_MASK: u8 = 0b00110000;

    let mut nums = [0_u16; 4];
    let mut nums_i = 3;

    let mut sum = 0;

    let mut i = 0;
    while i < dataset.len() {
        let c: &[u8; 2] = unsafe {
            dataset
                .get_unchecked(i..i + 2)
                .try_into()
                .unwrap_unchecked()
        };

        if c[1] & TYPE_MASK == DIGIT_MASK {
            *unsafe { nums.get_unchecked_mut(nums_i) } = u16::from_be_bytes(c.clone());
            i += 2;
        } else {
            *unsafe { nums.get_unchecked_mut(nums_i) } = c[0] as u16;
            i += 1;
        }

        if nums_i == 0 {
            let [a, b, c, d] = nums;
            let is_subset = a >= c && b <= d || c >= a && d <= b;
            if is_subset {
                sum += 1;
            }

            nums_i = 3;
        } else {
            nums_i -= 1;
        }
        i += 1;
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
