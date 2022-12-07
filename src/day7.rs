use aoc_runner_derive::aoc;

#[inline(always)]
fn bstoi(num: &[u8]) -> i64 {
    num.iter().fold(0, |a, &b| a * 10 + (b & 0xf) as i64)
}

#[aoc(day7, part1, optimized)]
pub fn day7_part1_optimized(dataset: &[u8]) -> i64 {
    const MAX_SIZE: i64 = 100_000;

    let mut cwd = Vec::<i64>::with_capacity(32);
    let mut total_size = 0;

    dataset
        .split(|&b| b == b'$')
        .skip(1)
        .for_each(|cmd_with_output| {
            let mut cmd_with_output =
                cmd_with_output[1..cmd_with_output.len() - 1].split(|&b| b == b'\n');

            let cmd = cmd_with_output.next().unwrap();

            if cmd[0] == b'c' {
                // cmd = cd [dir]
                if cmd[3] == b'.' {
                    let dir = cwd.pop().unwrap();
                    if dir <= MAX_SIZE {
                        total_size += dir;
                    }
                } else {
                    cwd.push(0);
                    println!("{}", cwd.len());
                }
            } else {
                // cmd = ls
                let dir_size = cmd_with_output
                    .filter(|fs_entry| fs_entry[0] != b'd')
                    .map(|file_entry| {
                        let size_str = file_entry.split(|&b| b == b' ').next().unwrap();
                        bstoi(size_str)
                    })
                    .sum::<i64>();

                cwd.iter_mut().for_each(|entry| *entry += dir_size);
            }
        });

    let extra_size = cwd
        .into_iter()
        .rev()
        .take_while(|&dir_size| dir_size <= MAX_SIZE)
        .sum::<i64>();

    total_size + extra_size
}

// #[aoc(day7, part2)]
// pub fn day7_part2(dataset: &[u8]) -> i64 {
//     todo!()
// }

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = br#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn test_bstoi() {
        assert_eq!(bstoi(b"8033020"), 8033020);
        assert_eq!(bstoi(b"8033020"), 8033020);
    }

    #[test]
    fn test_day7_part1() {
        assert_eq!(95437, day7_part1_optimized(INPUT));
    }

    // #[test]
    // fn test_day7_part2() {
    //     assert_eq!(0, day7_part2(INPUT));
    // }
}
