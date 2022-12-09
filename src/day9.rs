use aoc_runner_derive::aoc;

struct Bitset([u64; 16384]);

impl Bitset {
    #[inline(always)]
    fn get_and_set(&mut self, idx: usize) -> bool {
        let off_idx = idx / 64;
        let bit_idx = idx % 64;
        let is_set = self.0[off_idx] & (1 << bit_idx) != 0;
        self.0[off_idx] |= 1 << bit_idx;
        is_set
    }
}

#[aoc(day9, part1)]
pub fn day9_part1(dataset: &[u8]) -> i64 {
    let mut head: (i32, i32) = (512, 512);
    let mut tail: (i32, i32) = (512, 512);

    let mut set = Bitset([0_u64; 16384]);
    set.get_and_set(((tail.0 << 10) | tail.1) as usize);

    let mut count = 1;

    let mut i = 0;
    while i < dataset.len() - 3 {
        let (dist, dir);
        // The furthest element accessed in the slice is at i+3 and which is validated by the loop
        // condition
        unsafe {
            dir = *dataset.get_unchecked(i);
            if dataset[i + 3] == b'\n' {
                dist = *dataset.get_unchecked(i + 2) & 0xf;
                i += 4;
            } else {
                dist = (*dataset.get_unchecked(i + 2) & 0xf) * 10
                    + (*dataset.get_unchecked(i + 3) & 0xf);
                i += 5;
            };
        }

        match dir {
            b'R' => {
                for _ in 0..2.min(dist) {
                    head.0 += 1;
                    if (head.0 - tail.0).abs() > 1 {
                        tail.0 = head.0 - 1;
                        tail.1 = head.1;
                        let id = (tail.0 << 10) | tail.1;
                        if !set.get_and_set(id as usize) {
                            count += 1;
                        }
                    }
                }
                for _ in 2..dist {
                    head.0 += 1;
                    tail.0 += 1;
                    tail.1 = head.1;
                    let id = (tail.0 << 10) | tail.1;
                    if !set.get_and_set(id as usize) {
                        count += 1;
                    }
                }
            }
            b'L' => {
                for _ in 0..2.min(dist) {
                    head.0 -= 1;
                    if (head.0 - tail.0).abs() > 1 {
                        tail.0 = head.0 + 1;
                        tail.1 = head.1;
                        let id = (tail.0 << 10) | tail.1;
                        if !set.get_and_set(id as usize) {
                            count += 1;
                        }
                    }
                }
                for _ in 2..dist {
                    head.0 -= 1;
                    tail.0 -= 1;
                    tail.1 = head.1;
                    let id = (tail.0 << 10) | tail.1;
                    if !set.get_and_set(id as usize) {
                        count += 1;
                    }
                }
            }
            b'U' => {
                for _ in 0..2.min(dist) {
                    head.1 += 1;
                    if (head.1 - tail.1).abs() > 1 {
                        tail.1 = head.1 - 1;
                        tail.0 = head.0;
                        let id = (tail.0 << 10) | tail.1;
                        if !set.get_and_set(id as usize) {
                            count += 1;
                        }
                    }
                }
                for _ in 2..dist {
                    head.1 += 1;
                    tail.1 += 1;
                    tail.0 = head.0;
                    let id = (tail.0 << 10) | tail.1;
                    if !set.get_and_set(id as usize) {
                        count += 1;
                    }
                }
            }
            b'D' => {
                for _ in 0..2.min(dist) {
                    head.1 -= 1;
                    if (head.1 - tail.1).abs() > 1 {
                        tail.1 = head.1 + 1;
                        tail.0 = head.0;
                        let id = (tail.0 << 10) | tail.1;
                        if !set.get_and_set(id as usize) {
                            count += 1;
                        }
                    }
                }
                for _ in 2..dist {
                    head.1 -= 1;
                    tail.1 -= 1;
                    tail.0 = head.0;
                    let id = (tail.0 << 10) | tail.1;
                    if !set.get_and_set(id as usize) {
                        count += 1;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    count
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
