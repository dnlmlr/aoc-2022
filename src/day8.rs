use aoc_runner_derive::aoc;

#[inline(always)]
fn grid_count_visible(dataset: &[u8]) -> i64 {
    const N: usize = 99;
    const N1: usize = N + 1;
    assert_eq!(dataset.len(), N * N1);

    let mut matrix = [false; N * N];

    let mut lanes1 = [0; N];
    let mut lanes2 = [0; N];

    lanes1.copy_from_slice(&dataset[..N]);
    lanes2.copy_from_slice(&dataset[(N - 1) * N1..(N - 1) * N1 + N]);

    let mut idx1 = N + 1;
    let mut idx2 = N * N - N * 2 + 1;

    for y in 1..N - 1 {
        let window1: &[u8; N] = unsafe {
            dataset
                .get_unchecked(y * N1..y * N1 + N)
                .try_into()
                .unwrap_unchecked()
        };
        let window2: &[u8; N] = unsafe {
            dataset
                .get_unchecked((N - y - 1) * N1..(N - y - 1) * N1 + N)
                .try_into()
                .unwrap_unchecked()
        };

        let mut n1 = window1[0];
        let mut n2 = window2[N - 1];

        for x in 1..N - 1 {
            if window1[x] > lanes1[x] {
                lanes1[x] = window1[x];
                *unsafe { matrix.get_unchecked_mut(idx1) } = true;
            }
            if window2[x] > lanes2[x] {
                lanes2[x] = window2[x];
                *unsafe { matrix.get_unchecked_mut(idx2) } = true;
            }

            if window1[x] > n1 {
                n1 = window1[x];
                *unsafe { matrix.get_unchecked_mut(idx1) } = true;
            }
            if window2[N - 1 - x] > n2 {
                n2 = window2[N - 1 - x];
                *unsafe { matrix.get_unchecked_mut((N - y - 1) * N + (N - x - 1)) } = true;
            }
            idx1 += 1;
            idx2 += 1;
        }
        idx1 += 2;
        idx2 = idx2 + 2 - N * 2;
    }

    matrix.into_iter().filter(|&b| b).count() as i64 + (N * 4 - 4) as i64
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
