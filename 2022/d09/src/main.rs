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

fn part_2() {
    let input = include_str!("../input");

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

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::default();
    tail_positions.insert((0i32, 0i32));

    let mut knots_position: [(i32, i32); 10] = Default::default();

    for ((dx, dy), steps) in actions {
        for _ in 0..steps {

            knots_position[0].0 += dx;
            knots_position[0].1 += dy;

            for i in 1..10 {
                let (prev_knot, cur_knot) = knots_position.split_at_mut(i);
                let (prev_knot, cur_knot) = (prev_knot[i - 1], &mut cur_knot[0]);

                if prev_knot.0.abs_diff(cur_knot.0) > 1 || prev_knot.1.abs_diff(cur_knot.1) > 1 {
                    // Compute relative dx dy based on the previous knot
                    let (delta_x, delta_y) = (prev_knot.0 - cur_knot.0, prev_knot.1 - cur_knot.1);
                    let (dx, dy) = (delta_x.signum(), delta_y.signum());

                    *cur_knot = (cur_knot.0 + dx, cur_knot.1 + dy); // Previous position of the head
                }
            }
            tail_positions.insert(knots_position[knots_position.len() - 1]);
        }
    }

    println!("{}", tail_positions.len());

}

fn main() {
    part_1();
    part_2();
}
