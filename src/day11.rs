use aoc_runner_derive::aoc;

#[inline(always)]
fn bstoi2(s: &[u8; 3]) -> (i32, bool) {
    let num = (s[0] & 0xf) as i32 * 10 + (s[1] & 0xf) as i32;
    (num, s[2] == b',')
}

#[inline(always)]
fn bstoi12_lf(s: &[u8; 2]) -> (i32, usize) {
    if s[1] == b'\n' {
        ((s[0] & 0xf) as i32, 1)
    } else {
        ((s[0] & 0xf) as i32 * 10 + (s[1] & 0xf) as i32, 2)
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum Operation {
    AddConst(i32),
    MulConst(i32),
    #[default]
    AddOld,
    MulOld,
}

#[derive(Debug, Clone, Default)]
struct TestOp {
    divisor: i32,
    target_false: usize,
    target_true: usize,
}

#[derive(Debug, Clone, Default)]
struct Monkey {
    operation: Operation,
    test_op: TestOp,
    inspect_count: i64,
    item_count: u8,
    items: [u64; 32],
}

fn parse(dataset: &[u8]) -> [Monkey; 8] {
    let mut monkeys = [
        Monkey::default(),
        Monkey::default(),
        Monkey::default(),
        Monkey::default(),
        Monkey::default(),
        Monkey::default(),
        Monkey::default(),
        Monkey::default(),
    ];
    let mut insert = 0;

    let mut i = 0;
    while i < dataset.len() {
        i += 28;

        let mut items_count = 0;
        let mut items = [0; 32];
        loop {
            let (num, more_nums) = bstoi2(dataset[i..i + 3].try_into().unwrap());
            i += 4;
            items[items_count] = num as u64;
            items_count += 1;
            if !more_nums {
                break;
            }
        }
        // Skip until after "new = old ", next char should be the operator
        i += 22;

        let operation;
        if dataset[i] == b'*' {
            if dataset[i + 2] == b'o' {
                i += 27;
                operation = Operation::MulOld;
            } else {
                let (num, ii) = bstoi12_lf(dataset[i + 2..i + 2 + 2].try_into().unwrap());
                i += 2 + ii + 22;
                operation = Operation::MulConst(num);
            }
        } else {
            if dataset[i + 2] == b'o' {
                operation = Operation::AddOld;
            } else {
                let (num, ii) = bstoi12_lf(dataset[i + 2..i + 2 + 2].try_into().unwrap());
                i += 2 + ii + 22;
                operation = Operation::AddConst(num);
            }
        }

        let (divisor, ii) = bstoi12_lf(dataset[i..i + 2].try_into().unwrap());
        i += ii + 30;

        let target_true = (dataset[i] & 0xf) as usize;
        i += 32;
        let target_false = (dataset[i] & 0xf) as usize;
        i += 3;

        let test_op = TestOp {
            divisor,
            target_false,
            target_true,
        };

        let monkey = Monkey {
            item_count: items_count as u8,
            items,
            operation,
            test_op,
            inspect_count: 0,
        };

        monkeys[insert] = monkey;
        insert += 1;
    }
    monkeys
}

#[aoc(day11, part1)]
pub fn day11_part1(dataset: &[u8]) -> i64 {
    let mut monkeys = parse(dataset);

    for _ in 0..20 {
        for m in 0..8 {
            let monkey = unsafe { &mut *monkeys.as_mut_ptr().add(m) };
            while monkey.item_count > 0 {
                let item = unsafe { *monkey.items.get_unchecked(monkey.item_count as usize - 1) };
                monkey.item_count -= 1;
                let item = match monkey.operation {
                    Operation::AddConst(c) => item + c as u64,
                    Operation::MulConst(c) => item * c as u64,
                    Operation::AddOld => item * 2,
                    Operation::MulOld => item * item,
                } / 3;

                let mut target = if item % monkey.test_op.divisor as u64 == 0 {
                    unsafe { monkeys.get_unchecked_mut(monkey.test_op.target_true) }
                } else {
                    unsafe { monkeys.get_unchecked_mut(monkey.test_op.target_false) }
                };

                *unsafe { target.items.get_unchecked_mut(target.item_count as usize) } = item;
                target.item_count += 1;

                monkey.inspect_count += 1;
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspect_count);
    monkeys
        .into_iter()
        .rev()
        .map(|m| m.inspect_count)
        .take(2)
        .product::<i64>()
}

#[aoc(day11, part2)]
pub fn day11_part2(dataset: &[u8]) -> i64 {
    let mut monkeys = parse(dataset);

    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            let monkey = unsafe { &mut *monkeys.as_mut_ptr().add(m) };
            while monkey.item_count > 0 {
                let item = unsafe { *monkey.items.get_unchecked(monkey.item_count as usize - 1) };
                monkey.item_count -= 1;
                let item = match monkey.operation {
                    Operation::AddConst(c) => item + c as u64,
                    Operation::MulConst(c) => item * c as u64,
                    Operation::AddOld => item * 2,
                    Operation::MulOld => item * item,
                };

                let item = item % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19);

                let mut target = if item % monkey.test_op.divisor as u64 == 0 {
                    unsafe { monkeys.get_unchecked_mut(monkey.test_op.target_true) }
                } else {
                    unsafe { monkeys.get_unchecked_mut(monkey.test_op.target_false) }
                };

                *unsafe { target.items.get_unchecked_mut(target.item_count as usize) } = item;
                target.item_count += 1;

                monkey.inspect_count += 1;
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspect_count);
    monkeys
        .into_iter()
        .rev()
        .map(|m| m.inspect_count)
        .take(2)
        .product::<i64>()
}

/// Playing around with detecting repeating cycles. THIS DOES NOT WORK ATM!
#[aoc(day11, part2, experimental)]
pub fn day11_part2_experimental(dataset: &[u8]) -> i64 {
    let monkeys = parse(dataset);

    let mut counts = [0; 8];

    let mut items = monkeys
        .iter()
        .enumerate()
        .map(|(mi, m)| {
            m.items
                .iter()
                .take(m.item_count as usize)
                .map(move |&item| (mi, item))
        })
        .flatten()
        .collect::<Vec<_>>();

    let operations = monkeys
        .iter()
        .map(|m| {
            let op: Box<dyn Fn(u64) -> (u64, usize) + Send + Sync> = match m.operation {
                Operation::AddConst(c) => Box::new(move |item: u64| {
                    let item = (item + c as u64) % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19);
                    (
                        item,
                        if item % m.test_op.divisor as u64 == 0 {
                            m.test_op.target_true
                        } else {
                            m.test_op.target_false
                        },
                    )
                }),
                Operation::MulConst(c) => Box::new(move |item: u64| {
                    let item = (item * c as u64) % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19);
                    (
                        item,
                        if item % m.test_op.divisor as u64 == 0 {
                            m.test_op.target_true
                        } else {
                            m.test_op.target_false
                        },
                    )
                }),
                Operation::MulOld => Box::new(|item: u64| {
                    let item = (item * item) % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19);
                    (
                        item,
                        if item % m.test_op.divisor as u64 == 0 {
                            m.test_op.target_true
                        } else {
                            m.test_op.target_false
                        },
                    )
                }),
                Operation::AddOld => unreachable!(),
            };
            op
        })
        .collect::<Vec<_>>();

    items.iter_mut().for_each(|(mi, item)| {
        let mut cycle_detection: Vec<(usize, u64, i64)> = Vec::with_capacity(100);
        let mut cc = 10_000;
        for c in 0..10_000 {
            let mut cycle = false;
            loop {
                let last_mi = *mi;

                let (transformed_item, new_mi) = unsafe { operations.get_unchecked(*mi) }(*item);
                let id = (
                    last_mi,
                    transformed_item % monkeys[*mi].test_op.divisor as u64,
                    0,
                );

                if let Some(&cd) = cycle_detection.first() {
                    if cd.0 == id.0 && cd.1 == id.1 {
                        cycle = true;
                    }
                }
                cycle_detection.push(id);

                *item = transformed_item;
                *mi = new_mi;

                *unsafe { counts.get_unchecked_mut(*mi) } += 1;

                if *mi < last_mi {
                    cycle_detection.last_mut().unwrap().2 = 1;
                    break;
                }
            }
            if cycle {
                cc = c;
                break;
            }
        }
        'outer: loop {
            for iter in &cycle_detection {
                counts[iter.0] += 1;
                cc += iter.2;
                if cc > 10_001 {
                    break 'outer;
                }
            }
        }
    });

    counts.sort_unstable();
    counts.into_iter().rev().take(2).product::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = br#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#;

    #[test]
    fn test_day10_part1() {
        assert_eq!(10605, day11_part1(INPUT));
    }
}
