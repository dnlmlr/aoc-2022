use aoc_runner_derive::aoc;

#[inline(always)]
fn grid_count_visible(dataset: &[u8]) -> i64 {
    const N: usize = 99;
    const N1: usize = N + 1;

    let mut matrix = [false; N * N];

    let mut lanes = [0; N];
    for y in 0..N {
        let mut n = 0;
        for x in 0..N {
            let b = dataset[y * N1 + x];
            if b > n {
                n = b;
                matrix[y * N + x] = true;
            }
            if b > lanes[x] {
                lanes[x] = b;
                matrix[y * N + x] = true;
            }
        }
    }

    let mut lanes = [0; N];
    for y in (0..N).rev() {
        let mut n = 0;
        for x in (0..N).rev() {
            let b = dataset[y * N1 + x];
            if b > n {
                n = b;
                matrix[y * N + x] = true;
            }
            if b > lanes[x] {
                lanes[x] = b;
                matrix[y * N + x] = true;
            }
        }
    }

    matrix.into_iter().filter(|&b| b).count() as i64
}

#[aoc(day8, part1)]
pub fn day8_part1(dataset: &[u8]) -> i64 {
    grid_count_visible(dataset)
}

// #[aoc(day8, part2)]
// pub fn day8_part2(dataset: &[u8]) -> i64 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = br#"30373
25512
65332
33549
35390
"#;

    #[test]
    fn test_day8_part1() {
        assert_eq!(21, day8_part1(INPUT));
    }

    // #[test]
    // fn test_day7_part2() {
    //     assert_eq!(942298, day7_part2(include_bytes!("../input/2022/day7.txt")));
    // }
}
