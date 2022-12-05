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

#[aoc(day5, part1, optimized)]
pub fn day5_part1_optimized(dataset: &[u8]) -> String {
    let stacks = [0_u8; 64 * 9];
    let stack_idx = [0_usize; 9];

    unsafe {
        let s_ptr = stacks.as_ptr() as *mut u8;
        let sidx_ptr = stack_idx.as_ptr() as *mut usize;

        let mut ds_ptr = dataset.as_ptr().add(1);
        let ds_ptr_end = dataset.as_ptr_range().end;

        let mut k = 7;
        while *ds_ptr != b'1' {
            for i in 0..9 {
                let val = *ds_ptr;
                if val != b' ' {
                    *s_ptr.add((i << 6) + k) = val;
                    *sidx_ptr.add(i) += 1;
                }
                ds_ptr = ds_ptr.add(4);
            }
            k -= 1;
        }
        ds_ptr = ds_ptr.add(36);

        while ds_ptr < ds_ptr_end {
            ds_ptr = ds_ptr.add(5);

            let cd0 = *ds_ptr;
            let cd1 = *ds_ptr.add(1);

            let count;
            if cd1 == b' ' {
                ds_ptr = ds_ptr.add(7);
                count = (cd0 - b'0') as usize;
            } else {
                ds_ptr = ds_ptr.add(8);
                count = cd0 as usize * 10 + cd1 as usize - (b'0' as usize * 11);
            }

            let from = *ds_ptr as usize - b'0' as usize - 1;
            let to = *ds_ptr.add(5) as usize - b'0' as usize - 1;

            ds_ptr = ds_ptr.add(7);

            for _ in 0..count {
                *sidx_ptr.add(from) -= 1;

                *s_ptr.add((to << 6) + *sidx_ptr.add(to)) =
                    *s_ptr.add((from << 6) + *sidx_ptr.add(from));

                *sidx_ptr.add(to) += 1;
            }
        }
    }

    let mut out = String::with_capacity(9);

    for i in 0..9 {
        if stack_idx[i] == 0 {
            continue;
        }
        let val = stacks[i * 64 + stack_idx[i] - 1];
        out.push(val as char);
    }

    out
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
