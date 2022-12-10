use aoc_runner_derive::aoc;

#[inline(always)]
fn bstosi(s: &[u8; 3]) -> (i32, usize) {
    if s[0] == b'-' {
        let num = (s[1] & 0xf) as i32;
        if s[2] == b'\n' {
            (num * -1, 3)
        } else {
            (num * -10 - (s[2] & 0xf) as i32, 4)
        }
    } else {
        let num = (s[0] & 0xf) as i32;
        if s[1] == b'\n' {
            (num, 2)
        } else {
            (num * 10 + (s[1] & 0xf) as i32, 3)
        }
    }
}

#[aoc(day10, part1)]
pub fn day10_part1(dataset: &[u8]) -> i64 {
    let mut cycles_countdown = std::simd::Simd::from_array([0, 20]);
    let simd_delta = std::simd::Simd::from_array([1, -1]);
    let simd_delta2 = std::simd::Simd::from_array([2, -2]);

    let mut x = 1;

    let mut result = 0;

    let mut i = 0;
    while i < dataset.len() - 9 {
        while cycles_countdown[1] > 4 {
            if unsafe { *dataset.get_unchecked(i) } == b'n' {
                cycles_countdown += simd_delta;
                i += 5;
                continue;
            }

            let (val, ii) = bstosi(unsafe {
                dataset
                    .get_unchecked(i + 5..i + 8)
                    .try_into()
                    .unwrap_unchecked()
            });
            i += 5 + ii;

            cycles_countdown += simd_delta2;
            x += val;
            if unsafe { *dataset.get_unchecked(i) } == b'n' {
                cycles_countdown += simd_delta;
                i += 5;
                continue;
            }

            let (val, ii) = bstosi(unsafe {
                dataset
                    .get_unchecked(i + 5..i + 8)
                    .try_into()
                    .unwrap_unchecked()
            });
            i += 5 + ii;

            cycles_countdown += simd_delta2;
            x += val;
        }

        if unsafe { *dataset.get_unchecked(i) } == b'n' {
            cycles_countdown += simd_delta;
            if cycles_countdown[1] == 0 {
                result += cycles_countdown[0] * x;
                cycles_countdown[1] = 40;
            }
            i += 5;
            continue;
        }

        let (val, ii) = bstosi(unsafe {
            dataset
                .get_unchecked(i + 5..i + 8)
                .try_into()
                .unwrap_unchecked()
        });
        i += 5 + ii;

        cycles_countdown += simd_delta2;
        if cycles_countdown[1] <= 0 {
            result += (cycles_countdown[0] + cycles_countdown[1]) * x;
            cycles_countdown[1] += 40;
        }
        x += val;
    }

    result as i64
}

#[aoc(day10, part2)]
pub fn day10_part2(dataset: &[u8]) -> String {
    let mut cycles = 0_i32;
    let mut x = 1_i32;

    let mut result = vec![b'.'; 240];

    let mut i = 0;
    while i < dataset.len() - 9 * 2 {
        let mut c = cycles % 40;
        while c < 36 {
            if unsafe { *dataset.get_unchecked(i) } == b'n' {
                if (c - x).abs() < 2 {
                    result[cycles as usize] = b'#';
                }
                cycles += 1;
                c += 1;
                i += 5;
            } else {
                let (val, ii) = bstosi(unsafe {
                    dataset
                        .get_unchecked(i + 5..i + 8)
                        .try_into()
                        .unwrap_unchecked()
                });
                i += 5 + ii;

                if (c - x).abs() < 2 {
                    result[cycles as usize] = b'#';
                }
                cycles += 1;
                c += 1;
                if (c - x).abs() < 2 {
                    result[cycles as usize] = b'#';
                }
                cycles += 1;
                c += 1;

                x += val;
            }

            if unsafe { *dataset.get_unchecked(i) } == b'n' {
                if (c - x).abs() < 2 {
                    result[cycles as usize] = b'#';
                }
                cycles += 1;
                c += 1;
                i += 5;
            } else {
                let (val, ii) = bstosi(unsafe {
                    dataset
                        .get_unchecked(i + 5..i + 8)
                        .try_into()
                        .unwrap_unchecked()
                });
                i += 5 + ii;

                if (c - x).abs() < 2 {
                    result[cycles as usize] = b'#';
                }
                cycles += 1;
                c += 1;
                if (c - x).abs() < 2 {
                    result[cycles as usize] = b'#';
                }
                cycles += 1;
                c += 1;

                x += val;
            }
        }

        if unsafe { *dataset.get_unchecked(i) } == b'n' {
            if (cycles % 40 - x).abs() < 2 {
                result[cycles as usize] = b'#';
            }
            cycles += 1;
            i += 5;
        } else {
            let (val, ii) = bstosi(unsafe {
                dataset
                    .get_unchecked(i + 5..i + 8)
                    .try_into()
                    .unwrap_unchecked()
            });
            i += 5 + ii;

            if (cycles % 40 - x).abs() < 2 {
                result[cycles as usize] = b'#';
            }
            cycles += 1;
            if (cycles % 40 - x).abs() < 2 {
                result[cycles as usize] = b'#';
            }
            cycles += 1;

            x += val;
        }

        if unsafe { *dataset.get_unchecked(i) } == b'n' {
            if (cycles % 40 - x).abs() < 2 {
                result[cycles as usize] = b'#';
            }
            cycles += 1;
            i += 5;
        } else {
            let (val, ii) = bstosi(unsafe {
                dataset
                    .get_unchecked(i + 5..i + 8)
                    .try_into()
                    .unwrap_unchecked()
            });
            i += 5 + ii;

            if (cycles % 40 - x).abs() < 2 {
                result[cycles as usize] = b'#';
            }
            cycles += 1;
            if (cycles % 40 - x).abs() < 2 {
                result[cycles as usize] = b'#';
            }
            cycles += 1;

            x += val;
        }
    }

    unsafe { String::from_utf8_unchecked(result) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day10_part1() {
        assert_eq!(13140, day10_part1(INPUT));
    }

    #[test]
    fn test_day10_part2() {
        assert_eq!(
            [
                "###...##..###..#..#.####.#..#.####...##.",
                "#..#.#..#.#..#.#.#..#....#.#..#.......#.",
                "#..#.#..#.#..#.##...###..##...###.....#.",
                "###..####.###..#.#..#....#.#..#.......#.",
                "#....#..#.#....#.#..#....#.#..#....#..#.",
                "#....#..#.#....#..#.#....#..#.####..##..",
            ]
            .into_iter()
            .map(|line| line.chars())
            .flatten()
            .collect::<String>(),
            day10_part2(include_bytes!("../input/2022/day10.txt"))
        );
    }

    const INPUT: &[u8] = br#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;
}
