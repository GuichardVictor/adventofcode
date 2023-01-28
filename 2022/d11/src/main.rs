struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    divisor_test: usize,
    targets: (usize, usize),
    inserted: usize,
}

const N_ROUNDS: usize = 20;

fn part_1() {
    let input = include_str!("../input");

    let mut monkeys = input
        .split("\n\n")
        .map(|monkey_instruction| {
            let monkey_instruction = monkey_instruction
                .split("\n")
                .skip(1)
                .map(|l| l.split(": ").last().unwrap())
                .collect::<Vec<&str>>();

            Monkey {
                items: monkey_instruction[0]
                    .split(", ")
                    .map(|number| number.parse().unwrap())
                    .collect(),
                operation: {
                    let op: Vec<_> = monkey_instruction[1]
                        .split_once("= ")
                        .unwrap()
                        .1
                        .split(" ")
                        .collect();

                    match op[2] {
                        "old" => Box::new(|old| old * old),
                        value => match (op[1], value.parse::<usize>().unwrap()) {
                            ("+", v) => Box::new(move |old| old + v),
                            (_, v) => Box::new(move |old| old * v),
                        },
                    }
                },
                divisor_test: monkey_instruction[2]
                    .split_once("by ")
                    .unwrap()
                    .1
                    .parse()
                    .unwrap(),
                targets: (
                    monkey_instruction[3]
                        [monkey_instruction[3].len() - 1..monkey_instruction[3].len()]
                        .parse()
                        .unwrap(),
                    monkey_instruction[4]
                        [monkey_instruction[4].len() - 1..monkey_instruction[4].len()]
                        .parse()
                        .unwrap(),
                ),
                inserted: 0,
            }
        })
        .collect::<Vec<Monkey>>();

    for _ in 0..N_ROUNDS {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            monkey.inserted += monkey.items.len();

            let ok_index = monkey.targets.0.clone();
            let nok_index = monkey.targets.1.clone();

            let mut iterator = monkey.items.clone();
            monkey.items.clear();

            iterator.drain(..).for_each(|item| {
                let new_value = (monkeys[i].operation)(item) / 3;

                match new_value % monkeys[i].divisor_test {
                    0 => monkeys[ok_index].items.push(new_value),
                    _ => monkeys[nok_index].items.push(new_value),
                }
            });
        }
    }

    let mut counts = monkeys.iter().map(|m| m.inserted).collect::<Vec<usize>>();
    counts.sort();
    let result = counts.iter().rev().take(2).product::<usize>();

    println!("{}", result);
}

fn main() {
    part_1();
}
