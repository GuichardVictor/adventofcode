fn part_1() {
    let input = include_str!("../input");

    let (initial_state, instruction) = input.split_once("\n\n").unwrap();
    let mut stacks: [Vec<u8>; 9usize] = Default::default();

    initial_state
        .split(|b| b == '\n')
        .rev() // Fill the stack from the bottom
        .skip(1) // Skip the stack number
        .for_each(|line| {
            line.as_bytes()
                .iter()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| c != &&b' ')
                .for_each(|(i, c)| stacks[i].push(*c));
        });

    instruction
        .split("\n")
        .map(|line| {
            let inst: Vec<usize> = line
                .split(" ")
                .skip(1)
                .step_by(2)
                .map(|n| str::parse(n).unwrap())
                .collect::<Vec<usize>>();

            (inst[0], inst[1] - 1, inst[2] - 1) // -1 as given input is not 0 indexed
        })
        .for_each(|(n, from, to)| {
            for _ in 0..n {
                let item = stacks[from].pop().unwrap();
                stacks[to].push(item);
            }
        });

    stacks
        .iter()
        .for_each(|stack| print!("{}", *stack.last().unwrap() as char));
    println!();
}

fn main() {
    part_1();
}
