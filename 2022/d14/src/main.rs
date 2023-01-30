use std::collections::HashSet;

fn simulate_sand(points: &HashSet<(i32, i32)>, max_height: i32) -> Option<(i32, i32)> {
    let mut sand_position: (i32, i32) = (500, 0);

    loop {
        if sand_position.1 > max_height {
            return None;
        }

        let mut sand_simulation_iterator = [(0, 1), (-1, 1), (1, 1)]
            .iter()
            .map(|(dx, dy)| (sand_position.0 + dx, sand_position.1 + dy))
            .filter(|p| !points.contains(p));

        let next_point = sand_simulation_iterator.next();

        if let Some(candidate) = next_point {
            sand_position = candidate;
        } else {
            break;
        }
    }

    Some(sand_position)
}

fn part_1() {
    let input = include_str!("../input");

    let lines: Vec<Vec<(i32, i32)>> = input
        .split("\n")
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(",").unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let mut all_points = lines
        .iter()
        .map(|line| {
            line.windows(2)
                .map(|points| {
                    let (p1, p2) = (points[0], points[1]);

                    let (dx, dy) = ((p2.0 - p1.0).signum(), (p2.1 - p1.1).signum());

                    let mut points = vec![];
                    let mut current_point = p1.clone();
                    points.push(current_point);

                    while current_point != p2 {
                        current_point.0 += dx;
                        current_point.1 += dy;

                        points.push(current_point);
                    }

                    points
                })
                .flatten()
                .collect::<HashSet<(i32, i32)>>()
        })
        .flatten()
        .collect::<HashSet<(i32, i32)>>();

    let max_height = all_points.iter().max_by_key(|x| x.1).unwrap().1;
    let mut counter = 0;
    loop {
        match simulate_sand(&all_points, max_height) {
            Some(p) => {
                all_points.insert(p);
            }
            None => break,
        }
        counter += 1;
    }

    println!("{}", counter);
}

fn main() {
    part_1();
}
