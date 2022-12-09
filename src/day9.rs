use aoc_runner_derive::aoc;

struct Bitset([u64; (1024 * 1024) / 64]);

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
                if !set.get_and_set(id as usize) {
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
            if !set.get_and_set(id as usize) {
                count += 1;
            }
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
