use aoc_runner_derive::aoc;

#[inline(always)]
fn grid_count_visible(dataset: &[u8]) -> i64 {
    const N: usize = 99;
    const N1: usize = N + 1;

    let mut matrix = [false; N * N];

    let mut lanes = [0; N];
    let mut lanes2 = [0; N];
    for y in 0..N {
        let mut n1 = 0;
        let mut n2 = 0;
        for x in 0..N {
            let b1 = unsafe { *dataset.get_unchecked(y * N1 + x) };
            let b2 = unsafe { *dataset.get_unchecked((N - y - 1) * N1 + (N - x - 1)) };

            let idx1 = y * N + x;
            let idx2 = (N - y - 1) * N + (N - x - 1);

            if b1 > n1 {
                n1 = b1;
                matrix[idx1] = true;
            }
            if b1 > lanes[x] {
                lanes[x] = b1;
                matrix[idx1] = true;
            }

            if b2 > n2 {
                n2 = b2;
                matrix[idx2] = true;
            }
            if b2 > lanes2[x] {
                lanes2[x] = b2;
                matrix[idx2] = true;
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
