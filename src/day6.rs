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

#[aoc(day6, part1, optimized)]
pub fn day6_part1_optimized(dataset: &[u8]) -> i64 {
    let mut window_flags = [0_u8; 32];

    let mut num_duplicates = 0;

    for i in 0..4 {
        let idx = (dataset[i] & 0b11111) as usize;
        window_flags[idx] += 1;
        if window_flags[idx] > 1 {
            num_duplicates += 1;
        }
    }

    if num_duplicates == 0 {
        return 4;
    }

    for i in 4..dataset.len() {
        // idx is masked with 31, which means it can't be larger than 31. Therefore it will always
        // be a valid array index
        let idx = (dataset[i] & 0b11111) as usize;
        *unsafe { window_flags.get_unchecked_mut(idx) } += 1;
        if unsafe { *window_flags.get_unchecked(idx) } > 1 {
            num_duplicates += 1;
        }

        // idx is masked with 31, which means it can't be larger than 31. Therefore it will always
        // be a valid array index
        let idx = (dataset[i - 4] & 0b11111) as usize;
        *unsafe { window_flags.get_unchecked_mut(idx) } -= 1;
        if unsafe { *window_flags.get_unchecked(idx) } >= 1 {
            num_duplicates -= 1;
        }

        if num_duplicates == 0 {
            return i as i64 + 1;
        }
    }
    return -1;
}

#[aoc(day6, part2)]
pub fn day6_part2(dataset: &[u8]) -> i64 {
    find_unique_window_pos(dataset, 14).unwrap() as i64
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = br#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;

    #[test]
    fn test_day6_part1() {
        assert_eq!(7, day6_part1(INPUT));
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(19, day6_part2(INPUT));
    }
}
