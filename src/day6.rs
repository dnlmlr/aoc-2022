use aoc_runner_derive::aoc;

fn find_unique_window_pos(dataset: &[u8], window_size: usize) -> Option<usize> {
    'outer: for i in window_size..=dataset.len() {
        let window = &dataset[i - window_size..i];
        for w in 0..window.len() {
            if window[w + 1..].contains(&window[w]) {
                continue 'outer;
            }
        }
        return Some(i);
    }
    return None;
}

#[aoc(day6, part1)]
pub fn day6_part1(dataset: &[u8]) -> i64 {
    find_unique_window_pos(dataset, 4).unwrap() as i64
}

#[aoc(day6, part2)]
pub fn day6_part2(dataset: &[u8]) -> i64 {
    find_unique_window_pos(dataset, 14).unwrap() as i64
}

/// Returns 0 when no unique window is found
#[inline(always)]
fn find_unique_window_pos_optimized(dataset: &[u8], window_size: usize) -> usize {
    let mut window_flags = 0_u32;

    let mut num_count = 0;

    let mut i = 0;
    while i < window_size {
        let idx = 1 << (*unsafe { dataset.get_unchecked(i) } & 0b0001_1111) as usize;
        if window_flags & idx != 0 {
            num_count += 1;
        }
        window_flags ^= idx;

        i += 1;
    }

    while num_count != 0 {
        let idx1 = 1 << (*unsafe { dataset.get_unchecked(i) } & 0b11111) as usize;
        let idx2 = 1 << (*unsafe { dataset.get_unchecked(i - window_size) } & 0b11111) as usize;

        if window_flags & idx1 != 0 {
            num_count += 1;
        }
        window_flags ^= idx1;

        if window_flags & idx2 == 0 {
            num_count -= 1;
        }
        window_flags ^= idx2;

        i += 1;
    }
    return i;
}

#[aoc(day6, part1, optimized)]
pub fn day6_part1_optimized(dataset: &[u8]) -> i64 {
    find_unique_window_pos_optimized(dataset, 4) as i64
}

#[aoc(day6, part2, optimized)]
pub fn day6_part2_optimized(dataset: &[u8]) -> i64 {
    find_unique_window_pos_optimized(dataset, 14) as i64
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = br#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;

    #[test]
    fn test_day6_part1() {
        assert_eq!(7, day6_part1_optimized(INPUT));
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(19, day6_part2(INPUT));
    }
}
