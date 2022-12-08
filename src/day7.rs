use aoc_runner_derive::aoc;

#[inline(always)]
fn stoi_456(s: [u8; 6]) -> u64 {
    if s[5] == b' ' {
        s[4] as u64
            + s[3] as u64 * 10
            + s[2] as u64 * 100
            + s[1] as u64 * 1000
            + s[0] as u64 * 10000
            - (b'0' as u64 * 11111)
    } else if s[5] & 0b0100_0000 == 0 {
        s[5] as u64
            + s[4] as u64 * 10
            + s[3] as u64 * 100
            + s[2] as u64 * 1000
            + s[1] as u64 * 10000
            + s[0] as u64 * 100000
            - (b'0' as u64 * 111111)
    } else {
        s[3] as u64 + s[2] as u64 * 10 + s[1] as u64 * 100 + s[0] as u64 * 1000
            - (b'0' as u64 * 1111)
    }
}

// #[inline(always)]
// fn stoi(s: &[u8]) -> u64 {
//     s.iter()
//         .take_while(|&&b| b != b' ')
//         .fold(0, |a, &b| a * 10 + (b & 0xf) as u64)
// }

#[inline(always)]
fn parse(dataset: &[u8]) -> Vec<u32> {
    let mut dirs = Vec::with_capacity(200);
    let mut stack = Vec::with_capacity(16);

    let mut i = 0;
    while i < dataset.len() {
        let window: [u8; 6] = unsafe { dataset[i..i + 6].try_into().unwrap_unchecked() };

        if window[2] & 0b_0100_0000 == 0 {
            // number (file entry)
            // let num = stoi(&window);
            let num = stoi_456(window) as u32;
            *dirs.last_mut().unwrap() += num;
            i += 6;
        } else {
            if b'c' == window[2] {
                if window[5] == b'.' {
                    // cd ..
                    // println!("cd ..")
                    unsafe {
                        let from = stack.pop().unwrap_unchecked();
                        let to = *stack.last().unwrap_unchecked();
                        *dirs.get_unchecked_mut(to) += *dirs.get_unchecked_mut(from);
                    }
                    i += 5;
                } else {
                    // cd [dirname]
                    // println!("cd >>");
                    stack.push(dirs.len());
                    dirs.push(0);
                    i += 9;
                }
            }
        }
        i += 1;
        while dataset[i - 1] != b'\n' {
            i += 1;
        }
    }

    while stack.len() > 1 {
        let from = stack.pop().unwrap();
        let to = *stack.last().unwrap();
        dirs[to] += dirs[from];
    }

    dirs
}

#[aoc(day7, part1, optimized)]
pub fn day7_part1_optimized(dataset: &[u8]) -> i64 {
    const MAX_SIZE: u32 = 100_000;

    let dirs = parse(dataset);

    dirs.iter()
        .copied()
        .filter(|&entry| entry <= MAX_SIZE)
        .sum::<u32>() as i64
}

#[aoc(day7, part2, optimized)]
pub fn day7_part2_optimized(dataset: &[u8]) -> i64 {
    let dirs = parse(dataset);

    let free = 70000000 - dirs[0];
    let need_to_free = 30000000 - free;

    let mut smallest_dir = dirs[0];

    dirs.iter().copied().for_each(|d| {
        if d >= need_to_free && d < smallest_dir {
            smallest_dir = d;
        }
    });
    smallest_dir as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day7_part1() {
        assert_eq!(
            1443806,
            day7_part1_optimized(include_bytes!("../input/2022/day7.txt"))
        );
    }

    #[test]
    fn test_day7_part2() {
        assert_eq!(
            942298,
            day7_part2_optimized(include_bytes!("../input/2022/day7.txt"))
        );
    }
}
