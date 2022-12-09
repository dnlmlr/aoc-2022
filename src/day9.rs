use aoc_runner_derive::aoc;

#[inline(always)]
fn bstoi12(s: &[u8]) -> u8 {
    if s.len() == 1 {
        s[0] & 0xf
    } else {
        (s[0] & 0xf) * 10 + (s[1] & 0xf)
    }
}

#[aoc(day9, part1)]
pub fn day9_part1(dataset: &[u8]) -> i64 {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    let mut set = std::collections::HashSet::<(i32, i32)>::new();

    dataset
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let dir = line[0];
            let dist = bstoi12(&line[2..]);
            match dir {
                b'R' => {
                    for _ in 0..dist {
                        head.0 += 1;
                        if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                            tail.0 = head.0 - 1;
                            tail.1 = head.1;
                        }
                        set.insert(tail);
                    }
                }
                b'L' => {
                    for _ in 0..dist {
                        head.0 -= 1;
                        if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                            tail.0 = head.0 + 1;
                            tail.1 = head.1;
                        }
                        set.insert(tail);
                    }
                }
                b'U' => {
                    for _ in 0..dist {
                        head.1 += 1;
                        if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                            tail.1 = head.1 - 1;
                            tail.0 = head.0;
                        }
                        set.insert(tail);
                    }
                }
                b'D' => {
                    for _ in 0..dist {
                        head.1 -= 1;
                        if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                            tail.1 = head.1 + 1;
                            tail.0 = head.0;
                        }
                        set.insert(tail);
                    }
                }
                _ => unreachable!(),
            }
        });
    set.len() as i64
}

// #[aoc(day8, part2)]
// pub fn day8_part2(dataset: &[u8]) -> i64 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = br#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    #[test]
    fn test_day9_part1() {
        assert_eq!(13, day9_part1(INPUT));
    }
}
