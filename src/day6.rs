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
