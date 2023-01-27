fn part_1() {
    // Cycles: 20th, 60th, 100th, 140th, 180th, and 220th
    let input = include_str!("../input");
    let instructions = input.split("\n").map(|line| {
        match line {
            "noop" => (1, 0),
            _ => (2, line.split_once(" ").unwrap().1.parse::<i32>().unwrap()),
        }
    });

    let mut accumulator = 0i32;
    let mut register_x = 1i32;
    let mut current_cycle = 0i32;

    instructions.for_each(|(cycles, add_value)| {
        for _ in 1..cycles+1 {
            current_cycle += 1;

            if current_cycle % 40 == 20 {
                accumulator += current_cycle * register_x;
            }
        }

        register_x += add_value;
    });

    println!("{}", accumulator);
}

fn main() {
    part_1();
}
