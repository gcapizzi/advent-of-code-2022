use anyhow::Result;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: fn(u64) -> u64,
    test_divisible: u64,
    test_if_true: usize,
    test_if_false: usize,
    inspections: u64,
}

fn main() -> Result<()> {
    let monkeys = vec![
        Monkey {
            items: vec![92, 73, 86, 83, 65, 51, 55, 93],
            operation: |old| old * 5,
            test_divisible: 11,
            test_if_true: 3,
            test_if_false: 4,
            inspections: 0,
        },
        Monkey {
            items: vec![99, 67, 62, 61, 59, 98],
            operation: |old| old * old,
            test_divisible: 2,
            test_if_true: 6,
            test_if_false: 7,
            inspections: 0,
        },
        Monkey {
            items: vec![81, 89, 56, 61, 99],
            operation: |old| old * 7,
            test_divisible: 5,
            test_if_true: 1,
            test_if_false: 5,
            inspections: 0,
        },
        Monkey {
            items: vec![97, 74, 68],
            operation: |old| old + 1,
            test_divisible: 17,
            test_if_true: 2,
            test_if_false: 5,
            inspections: 0,
        },
        Monkey {
            items: vec![78, 73],
            operation: |old| old + 3,
            test_divisible: 19,
            test_if_true: 2,
            test_if_false: 3,
            inspections: 0,
        },
        Monkey {
            items: vec![50],
            operation: |old| old + 5,
            test_divisible: 7,
            test_if_true: 1,
            test_if_false: 6,
            inspections: 0,
        },
        Monkey {
            items: vec![95, 88, 53, 75],
            operation: |old| old + 8,
            test_divisible: 3,
            test_if_true: 0,
            test_if_false: 7,
            inspections: 0,
        },
        Monkey {
            items: vec![50, 77, 98, 85, 94, 56, 89],
            operation: |old| old + 2,
            test_divisible: 13,
            test_if_true: 4,
            test_if_false: 0,
            inspections: 0,
        },
    ];

    dbg!(monkey_business(&monkeys, 20, Some(3)));
    dbg!(monkey_business(&monkeys, 10_000, None));

    Ok(())
}

fn monkey_business(monkeys: &Vec<Monkey>, rounds: usize, relief_factor: Option<u64>) -> u64 {
    let mut ms = monkeys.clone();
    let lcm: u64 = monkeys.iter().map(|m| m.test_divisible).product();

    for _ in 0..rounds {
        for i in 0..ms.len() {
            let monkey = ms[i].clone();

            for item in &monkey.items {
                ms[i].inspections += 1;

                let mut worry_level = (monkey.operation)(*item);
                if let Some(rf) = relief_factor {
                    worry_level /= rf;
                } else {
                    worry_level %= lcm;
                }

                let next_monkey = if worry_level % monkey.test_divisible == 0 {
                    monkey.test_if_true
                } else {
                    monkey.test_if_false
                };

                ms[i].items = vec![];
                ms[next_monkey].items.push(worry_level);
            }
        }
    }

    let mut inspections: Vec<u64> = ms.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}
