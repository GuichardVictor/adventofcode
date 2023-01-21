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

fn part_2() {
    let input = include_str!("../input");

    let (initial_state, instruction) = input.split_once("\n\n").unwrap();
    let mut stacks: [Vec<u8>; 9usize] = Default::default();
    let mut temp_data = [0; 64];// 64 = 8 columns * 8 rows in the input data

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
            let len= stacks[from].len();
            let slice = &mut temp_data[..n]; // Take only the amount of required data
            slice.copy_from_slice(&stacks[from][len-n..len]); // Copy from the data that will be moved
            stacks[from].truncate(len - n); // Remove the removed data
            stacks[to].extend(slice.iter()); // Add it to the new location
        });

    stacks
        .iter()
        .for_each(|stack| print!("{}", *stack.last().unwrap() as char));
    println!();
}

fn main() {
    part_1();
    part_2();
}
