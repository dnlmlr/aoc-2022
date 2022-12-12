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
    items: Vec<i32>,
    operation: Operation,
    test_op: TestOp,
    inspect_count: i64,
}

fn parse(dataset: &[u8]) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let mut i = 0;
    while i < dataset.len() {
        i += 28;
        let mut items = Vec::new();
        loop {
            let (num, more_nums) = bstoi2(dataset[i..i + 3].try_into().unwrap());
            i += 4;
            items.push(num);
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
            items,
            operation,
            test_op,
            inspect_count: 0,
        };

        monkeys.push(monkey);
    }
    monkeys
}

#[aoc(day11, part1)]
pub fn day11_part1(dataset: &[u8]) -> i64 {
    let mut monkeys = parse(dataset);
    let mut monkey = Monkey::default();

    for _ in 0..20 {
        for m in 0..monkeys.len() {
            std::mem::swap(&mut monkeys[m], &mut monkey);
            while let Some(item) = monkey.items.pop() {
                let item = match monkey.operation {
                    Operation::AddConst(c) => item + c,
                    Operation::MulConst(c) => item * c,
                    Operation::AddOld => item * 2,
                    Operation::MulOld => item * item,
                } / 3;
                if item % monkey.test_op.divisor == 0 {
                    monkeys[monkey.test_op.target_true].items.push(item);
                } else {
                    monkeys[monkey.test_op.target_false].items.push(item);
                }
                monkey.inspect_count += 1;
            }
            std::mem::swap(&mut monkeys[m], &mut monkey);
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

// #[aoc(day11, part2)]
// pub fn day11_part2(dataset: &[u8]) -> i64 {
//     0
// }

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
