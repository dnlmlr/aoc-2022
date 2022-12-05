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

#[derive(Debug, Clone)]
struct Stack {
    data: [u8; 64],
    idx: usize,
}

impl Stack {
    fn new() -> Self {
        Self {
            data: [0; 64],
            idx: 0,
        }
    }

    fn reverse(&mut self) {
        self.data[..self.idx].reverse();
    }

    fn push(&mut self, val: u8) {
        self.data[self.idx] = val;
        self.idx += 1;
    }

    fn pop(&mut self) -> u8 {
        self.idx -= 1;
        self.data[self.idx]
    }

    fn last(&self) -> Option<u8> {
        (self.idx > 0).then(|| self.data[self.idx - 1])
    }
}

#[aoc(day5, part1, optimized)]
pub fn day5_part1_optimized(dataset: &[u8]) -> String {
    let mut stacks = [
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
        Stack::new(),
    ];

    let mut i = 0;

    dataset
        .array_chunks::<36>()
        .take_while(|line| line[1] != b'1')
        .for_each(|line| {
            i += 36;
            line.array_chunks::<4>().enumerate().for_each(|(k, c)| {
                if c[1] != b' ' {
                    stacks[k].push(c[1]);
                }
            });
        });

    for stack in &mut stacks {
        stack.reverse();
    }

    i += 37;
    while i < dataset.len() {
        i += 5;

        let count;
        if unsafe { *dataset.get_unchecked(i + 1) } == b' ' {
            count = (unsafe { *dataset.get_unchecked(i) } - b'0') as usize;
            i += 7;
        } else {
            count = unsafe { *dataset.get_unchecked(i) } as usize * 10
                + unsafe { *dataset.get_unchecked(i + 1) } as usize
                - (b'0' as usize * 11);
            i += 8;
        }

        let from = (unsafe { *dataset.get_unchecked(i) } - b'0') as usize - 1;
        let to = (unsafe { *dataset.get_unchecked(i + 5) } - b'0') as usize - 1;

        i += 7;

        for _ in 0..count {
            let val = stacks[from].pop();
            stacks[to].push(val);
        }
    }

    stacks
        .into_iter()
        .filter_map(|stack| stack.last())
        .map(|c| c as char)
        .collect::<String>()
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
