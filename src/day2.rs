use aoc_runner_derive::aoc;

#[inline(always)]
fn fold_game(line: &[u8]) -> u8 {
    let opponent = line[0] - b'A';
    let own = line[2] & 3;
    own << 2 | opponent
}

#[inline(always)]
fn map_sum(dataset: &[u8], map: [i64; 12]) -> i64 {
    let mut chunks = dataset.array_chunks::<4>();

    let sum = (&mut chunks)
        .map(|curr| {
            let combined = fold_game(curr);
            map[combined as usize]
        })
        .sum::<i64>();

    if chunks.remainder().is_empty() {
        sum
    } else {
        let combined = fold_game(chunks.remainder());
        sum + map[combined as usize]
    }
}

#[aoc(day2, part1)]
pub fn day2_part1(dataset: &[u8]) -> i64 {
    let map = [
        4, 1, 7, 0, //
        8, 5, 2, 0, //
        3, 9, 6, 0, //
    ];

    map_sum(dataset, map)
}

#[aoc(day2, part2)]
pub fn day2_part2(dataset: &[u8]) -> i64 {
    let map = [
        3, 1, 2, 0, //
        4, 5, 6, 0, //
        8, 9, 7, 0, //
    ];

    map_sum(dataset, map)
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = br#"A Y
B X
C Z"#;

    #[test]
    fn test_day2_part1() {
        assert_eq!(15, day2_part1(INPUT));
    }

    #[test]
    fn test_day2_part2() {
        assert_eq!(12, day2_part2(INPUT));
    }
}
