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

macro_rules! shift_window {
    ($dataset:ident, $window_flags:ident, $window_size:expr, $num_duplicates:expr, $offset:expr, $shift:expr) => {
        // idx is masked with 31, which means it can't be larger than 31. Therefore it will always
        // be a valid array index
        let idx1 = ($dataset[$offset + $shift] & 0b11111) as usize;
        let idx2 = ($dataset[$offset - $window_size + $shift] & 0b11111) as usize;

        if idx1 != idx2 {
            *unsafe { $window_flags.get_unchecked_mut(idx1) } += 1;
            if unsafe { *$window_flags.get_unchecked(idx1) } > 1 {
                $num_duplicates += 1;
            }

            // idx is masked with 31, which means it can't be larger than 31. Therefore it will always
            // be a valid array index
            *unsafe { $window_flags.get_unchecked_mut(idx2) } -= 1;
            if unsafe { *$window_flags.get_unchecked(idx2) } >= 1 {
                $num_duplicates -= 1;
            }

            if $num_duplicates == 0 {
                return $offset + 1 + $shift;
            }
        }
    };
}

/// Returns 0 when no unique window is found
#[inline(always)]
fn find_unique_window_pos_optimized(dataset: &[u8], window_size: usize) -> usize {
    let mut window_flags = [0_u8; 32];

    let mut num_duplicates = 0;

    for i in 0..window_size {
        let idx = (dataset[i] & 0b11111) as usize;
        window_flags[idx] += 1;
        if window_flags[idx] > 1 {
            num_duplicates += 1;
        }
    }

    if num_duplicates == 0 {
        return window_size;
    }

    let mut i = window_size;
    while i < dataset.len() {
        shift_window!(dataset, window_flags, window_size, num_duplicates, i, 0);
        shift_window!(dataset, window_flags, window_size, num_duplicates, i, 1);
        shift_window!(dataset, window_flags, window_size, num_duplicates, i, 2);
        shift_window!(dataset, window_flags, window_size, num_duplicates, i, 3);
        shift_window!(dataset, window_flags, window_size, num_duplicates, i, 4);
        shift_window!(dataset, window_flags, window_size, num_duplicates, i, 5);
        shift_window!(dataset, window_flags, window_size, num_duplicates, i, 6);
        shift_window!(dataset, window_flags, window_size, num_duplicates, i, 7);

        i += 8;
    }
    return 0;
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
        assert_eq!(7, day6_part1(INPUT));
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(19, day6_part2(INPUT));
    }
}
