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

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

fn part_2() {
    // Cycles: 20th, 60th, 100th, 140th, 180th, and 220th
    let input = include_str!("../input");
    let instructions = input.split("\n").map(|line| {
        match line {
            "noop" => (1, 0),
            _ => (2, line.split_once(" ").unwrap().1.parse::<i32>().unwrap()),
        }
    });

    let mut register_x = 1i32;
    let mut current_cycle = 0usize;
    let mut ctr = [[false; WIDTH]; HEIGHT];

    instructions.for_each(|(cycles, add_value)| {
        for _ in 1..cycles+1 {
            let (x, y) = ((current_cycle) % WIDTH, (current_cycle) / WIDTH % HEIGHT);
            ctr[y][x] = (x as i32 - register_x).abs() <= 1;

            current_cycle += 1;
        }

        register_x += add_value;
    });

    ctr.iter().for_each(|line| {
        line.iter().for_each(|pixel| {
            if *pixel {print!("#")} else {print!(" ")}
        });
        print!("\n");
    })
}

fn main() {
    part_1();
    part_2();
}
