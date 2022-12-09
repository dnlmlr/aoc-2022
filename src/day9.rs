use aoc_runner_derive::aoc;

struct Bitset([u64; (1024 * 1024) / 64]);

impl Bitset {
    #[inline(always)]
    fn get_and_set(&mut self, idx: usize) -> bool {
        let off_idx = idx >> 6;
        let bit_idx = idx & 0b111111;
        let is_set = (self.0[off_idx] >> bit_idx) & 1 == 1;
        self.0[off_idx] |= 1 << bit_idx;
        is_set
    }

    #[inline(always)]
    unsafe fn get_and_set_unchecked(&mut self, idx: usize) -> bool {
        let off_idx = idx >> 6;
        let bit_idx = idx & 0b111111;
        let chunk = self.0.get_unchecked_mut(off_idx);
        let is_set = (*chunk >> bit_idx) & 1 == 1;
        *chunk |= 1 << bit_idx;
        is_set
    }
}

#[aoc(day9, part1)]
pub fn day9_part1(dataset: &[u8]) -> i64 {
    let mut head: (i32, i32) = (511, 511);
    let mut tail: (i32, i32) = (511, 511);

    let mut set = Bitset([0_u64; (1024 * 1024) / 64]);
    set.get_and_set(((tail.0 << 10) | tail.1) as usize);

    let mut count = 1;

    let mut i = 0;
    while i < dataset.len() - 3 {
        let (dist, dir);
        // The furthest element accessed in the slice is at i+3 and which is validated by the loop
        // condition
        unsafe {
            dir = *dataset.get_unchecked(i);
            let d1 = *dataset.get_unchecked(i + 2);
            let d2 = *dataset.get_unchecked(i + 3);
            if d2 == b'\n' {
                dist = d1 & 0xf;
                i += 4;
            } else {
                dist = (d1 & 0xf) * 10 + (d2 & 0xf);
                i += 5;
            };
        }

        let (dx, dy) = if dir == b'R' {
            (1, 0)
        } else if dir == b'L' {
            (-1, 0)
        } else if dir == b'U' {
            (0, 1)
        } else {
            (0, -1)
        };

        for _ in 0..2.min(dist) {
            head.0 += dx;
            head.1 += dy;
            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                tail.0 = head.0 - dx;
                tail.1 = head.1 - dy;
                let id = (tail.0 << 10) | tail.1;
                if unsafe { !set.get_and_set_unchecked(id as usize) } {
                    count += 1;
                }
            }
        }

        for _ in 2..dist {
            head.0 += dx;
            head.1 += dy;
            tail.0 = head.0 - dx;
            tail.1 = head.1 - dy;
            let id = (tail.0 << 10) | tail.1;
            if unsafe { !set.get_and_set_unchecked(id as usize) } {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day9, part2)]
pub fn day9_part2(dataset: &[u8]) -> i64 {
    let mut rope = [(511_i32, 511_i32); 10];

    let mut set = Bitset([0_u64; (1024 * 1024) / 64]);
    set.get_and_set(((rope[0].0 << 10) | rope[0].1) as usize);

    let mut count = 1;

    let mut i = 0;
    while i < dataset.len() - 3 {
        let (dist, dir);
        // The furthest element accessed in the slice is at i+3 and which is validated by the loop
        // condition
        unsafe {
            dir = *dataset.get_unchecked(i);
            let d1 = *dataset.get_unchecked(i + 2);
            let d2 = *dataset.get_unchecked(i + 3);
            if d2 == b'\n' {
                dist = d1 & 0xf;
                i += 4;
            } else {
                dist = (d1 & 0xf) * 10 + (d2 & 0xf);
                i += 5;
            };
        }

        let (dx, dy) = if dir == b'R' {
            (1, 0)
        } else if dir == b'L' {
            (-1, 0)
        } else if dir == b'U' {
            (0, 1)
        } else {
            (0, -1)
        };

        for _ in 0..dist {
            rope[0].0 += dx;
            rope[0].1 += dy;
            for i in 1..rope.len() {
                let dr = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if dr.0.abs() > 1 || dr.1.abs() > 1 {
                    rope[i].0 += dr.0.signum();
                    rope[i].1 += dr.1.signum();
                }
            }
            let id = (rope[rope.len() - 1].0 << 10) | rope[rope.len() - 1].1;
            if unsafe { !set.get_and_set_unchecked(id as usize) } {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &[u8] = br#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    const INPUT2: &[u8] = br#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    fn test_day9_part1() {
        assert_eq!(13, day9_part1(INPUT1));
    }
    #[test]
    fn test_day9_part2() {
        assert_eq!(36, day9_part2(INPUT2));
    }
}
