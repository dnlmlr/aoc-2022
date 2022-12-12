use std::collections::VecDeque;

use aoc_runner_derive::aoc;

#[inline(always)]
fn can_move(dataset: &[u8], pos: usize, other: usize) -> bool {
    if unsafe { *dataset.get_unchecked(other) } & 0b1000_0000_u8 != 0 {
        return false;
    }

    let val1 = match unsafe { *dataset.get_unchecked(pos) } & 0b0111_1111 {
        b'S' => b'a' - 1,
        b'E' => b'z',
        b => b,
    } as i8;

    let val2 = match unsafe { *dataset.get_unchecked(other) } & 0b0111_1111 {
        b'S' => b'a' - 1,
        b'E' => b'z',
        b => b,
    } as i8;

    (val2 - val1) <= 1
}

#[aoc(day12, part1)]
pub fn day12_part1(dataset: &[u8]) -> i64 {
    let width = dataset.iter().position(|&b| b == b'\n').unwrap();
    let height = dataset.len() / (width + 1);

    let mut cop = vec![0xff_u8; (width + 2) * (height + 2)];
    for y in 0..height {
        cop[(y + 1) * (width + 2) + 1..(y + 1) * (width + 2) + 1 + width]
            .copy_from_slice(&dataset[y * (width + 1)..y * (width + 1) + width]);
    }

    let start = cop.iter().position(|&b| b == b'S').unwrap();
    let end = cop.iter().position(|&b| b == b'E').unwrap();

    let mut queue = VecDeque::<(usize, u32)>::new();
    queue.push_back((start, 0));
    cop[start] |= 0b1000_0000_u8;

    while let Some((curr, depth)) = queue.pop_front() {
        if curr == end {
            return depth as i64;
        }

        if can_move(&cop, curr, curr - 1) {
            *unsafe { cop.get_unchecked_mut(curr - 1) } |= 0b1000_0000_u8;
            queue.push_back((curr - 1, depth + 1));
        }
        if can_move(&cop, curr, curr + 1) {
            *unsafe { cop.get_unchecked_mut(curr + 1) } |= 0b1000_0000_u8;
            queue.push_back((curr + 1, depth + 1));
        }
        if can_move(&cop, curr, curr + width + 1 + 1) {
            *unsafe { cop.get_unchecked_mut(curr + width + 1 + 1) } |= 0b1000_0000_u8;
            queue.push_back((curr + width + 1 + 1, depth + 1));
        }
        if can_move(&cop, curr, curr - width - 1 - 1) {
            *unsafe { cop.get_unchecked_mut(curr - width - 1 - 1) } |= 0b1000_0000_u8;
            queue.push_back((curr - width - 1 - 1, depth + 1));
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = br#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn test_day10_part1() {
        assert_eq!(31, day12_part1(INPUT));
    }
}
