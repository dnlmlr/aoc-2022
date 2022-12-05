use aoc_runner_derive::aoc;

/// Parse an ASCII string with either 1 or 2 characters to i64
#[inline(always)]
fn bstoi_2(num: &[u8]) -> i64 {
    if num.len() == 1 {
        (num[0] - b'0') as i64
    } else {
        num[0] as i64 * 10 + num[1] as i64 - (b'0' as i64 * 11)
    }
}

#[aoc(day5, part1)]
pub fn day5_part1(dataset: &[u8]) -> String {
    let mut stacks = vec![Vec::with_capacity(32); 9];
    let mut temp_stack = Vec::new();

    let mut lines = dataset.split_inclusive(|&b| b == b'\n');

    // Create the stacks
    (&mut lines)
        .take_while(|line| line[1] != b'1')
        .for_each(|line| {
            line.array_chunks::<4>().enumerate().for_each(|(i, chest)| {
                let chest_id = chest[1];
                if chest_id != b' ' {
                    stacks[i].push(chest_id as char);
                }
            })
        });

    // Fix the ordering
    stacks.iter_mut().for_each(|stack| stack.reverse());

    lines.next();

    lines.for_each(|line| {
        let mut cmd_segments = line.split(|&b| b == b' ' || b == b'\n');
        cmd_segments.next();
        let chest_count = bstoi_2(cmd_segments.next().unwrap()) as usize;
        cmd_segments.next();
        let from = bstoi_2(cmd_segments.next().unwrap()) as usize - 1;
        cmd_segments.next();
        let to = bstoi_2(cmd_segments.next().unwrap()) as usize - 1;

        std::mem::swap(&mut stacks[from], &mut temp_stack);
        let truncated_len = temp_stack.len() - chest_count;
        stacks[to].extend(temp_stack.drain(truncated_len..).rev());
        temp_stack.truncate(truncated_len);
        std::mem::swap(&mut stacks[from], &mut temp_stack);
    });

    stacks
        .iter()
        .filter_map(|stack| stack.last().map(|&c| c as char))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &[u8] = br#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_day5_part1() {
        assert_eq!("CMZ", day5_part1(INPUT));
    }

    // #[test]
    // fn test_day4_part2() {
    //     assert_eq!(4, day5_part2(INPUT));
    // }
}
