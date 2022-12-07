use aoc_runner_derive::aoc;

#[aoc(day7, part1, optimized)]
pub fn day7_part1_optimized(dataset: &[u8]) -> i64 {
    let dataset = if *dataset.last().unwrap() == b'\n' {
        &dataset[..dataset.len() - 1]
    } else {
        dataset
    };
    const MAX_SIZE: i64 = 100_000;

    let mut cwd = [0_i64; 16];
    let mut cwd_idx = 0;
    let mut total_size = 0;

    // skip "$ "
    let mut i = 2;

    while i < dataset.len() {
        // "cd [...]" has 'c' at current offset
        if dataset[i] == b'c' {
            // "cd .." has a '.' at +3
            if dataset[i + 3] == b'.' {
                cwd_idx -= 1;
                let dir = cwd[cwd_idx];
                if dir <= MAX_SIZE {
                    total_size += dir;
                }

                // Skip "cd ..[LF]$ "
                i += 8;
            }
            // must be "cd [DIR_NAME]"
            else {
                cwd[cwd_idx] = 0;
                cwd_idx += 1;

                // skip over "cd " and 2 more chars. If "[DIR_NAME]" is only 1 char, this skips to
                // the next line
                i += 5;

                // continue skipping until reaching the '$' which denotes the next command
                while dataset[i] != b'$' {
                    i += 1;
                }
                // skip over the "$ "
                i += 2;
            }
        }
        // if not c, it has to be a line with ls
        else {
            // skip over "ls[LF]"
            i += 3;

            let mut sum = 0;

            // parse lines until reaching the next command
            'outer: while dataset[i] != b'$' {
                // "dir [name]" starts with 'd'
                if dataset[i] == b'd' {
                    // skip over "dir " + 1 more
                    i += 5;
                } else {
                    // entry is "[size] [name]"

                    let mut num = (dataset[i] & 0x0f) as i64;

                    i += 1;
                    // parse digits until reaching the space
                    while dataset[i] != b' ' {
                        num = num * 10 + (dataset[i] & 0x0f) as i64;
                        i += 1;
                    }
                    sum += num;
                }

                // skip until reaching the newline
                while dataset[i] != b'\n' {
                    i += 1;
                    if i >= dataset.len() {
                        break 'outer;
                    }
                }

                // Skip the newline
                i += 1;
            }

            // Apply the newly detected sizes to all parent dirs
            // This range must be valid, since there already were inserts at `[cw_idx-1]`
            unsafe { cwd.get_unchecked_mut(..cwd_idx) }
                .iter_mut()
                .for_each(|entry| *entry += sum);

            // Skip over "$ "
            i += 2;
        }
    }

    // Get the matching sizes of directories still left in the cd stack
    let extra_size = cwd[..cwd_idx]
        .into_iter()
        .rev()
        .take_while(|&dir_size| dir_size <= &MAX_SIZE)
        .sum::<i64>();

    total_size + extra_size
}

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
    fn test_day7_part1() {
        assert_eq!(95437, day7_part1_optimized(INPUT));
    }

    // #[test]
    // fn test_day7_part2() {
    //     assert_eq!(0, day7_part2(INPUT));
    // }
}
