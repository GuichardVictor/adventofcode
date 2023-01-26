use std::collections::HashSet;

fn part_1() {
    let input = include_str!("../input");
    
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::default();
    tail_positions.insert((0i32, 0i32));

    let mut head_position = (0i32, 0i32);
    let mut tail_position = (0i32, 0i32);

    let actions = input.split('\n').map(|line| {
        match (
            line.chars().next().unwrap(),
            line[2..].parse::<i32>().unwrap(),
        ) {
            ('U', l) => ((0, -1), l),
            ('D', l) => ((0, 1), l),
            ('L', l) => ((-1, 0), l),
            (_, l) => ((1, 0), l),
        }
    });

    for ((dx, dy), steps) in actions {
        for _ in 0..steps {
            head_position = (head_position.0 + dx, head_position.1 + dy);
            if head_position.0.abs_diff(tail_position.0) > 1 || head_position.1.abs_diff(tail_position.1) > 1 {
                tail_position = (head_position.0 - dx, head_position.1 - dy); // Previous position of the head
                tail_positions.insert(tail_position);
            }
        }
    }

    println!("{}", tail_positions.len());
}

fn main() {
    part_1();
}
