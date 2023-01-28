use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    graph: &Vec<u8>,
    source: usize,
    destination: usize,
    width: usize,
    height: usize,
) -> usize {
    let mut priority_queue: BinaryHeap<State> = BinaryHeap::default();
    priority_queue.push(State {
        cost: 0,
        position: source,
    });

    let mut seen: HashSet<usize> = HashSet::default();

    while let Some(State { cost, position }) = priority_queue.pop() {
        if position == destination {
            return cost;
        }

        if seen.contains(&position) {
            continue;
        }

        seen.insert(position);

        // Add neighbors
        let pos_x = (position % width) as i32;
        let pos_y = (position / width) as i32;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = (pos_x + dx, pos_y + dy);

            if x < 0 || (x as usize) >= width {
                continue;
            }

            if y < 0 || (y as usize) >= height {
                continue;
            }

            let new_1d_pos = (y as usize) * width + (x as usize);

            if graph[new_1d_pos] > (graph[position] + 1) {
                continue;
            }

            priority_queue.push(State {
                cost: cost + 1,
                position: new_1d_pos,
            });
        }
    }

    return 0;
}

fn part_1() {
    let input = include_str!("../input");
    let mut height_map: Vec<_> = input
        .bytes()
        .filter(|b| b != &b'\n')
        .map(|b| b.to_ascii_lowercase() - b'a')
        .collect();

    let width = input.bytes().position(|b| b == b'\n').unwrap();
    let height = height_map.len() / width;

    let mut start = input.bytes().position(|b| b == b'S').unwrap();
    let mut end = input.bytes().position(|b| b == b'E').unwrap();

    // Change coordinate to height map coordinate and update values:
    (start, end) = (start - start / (width + 1), end - end / (width + 1));
    (height_map[start], height_map[end]) = (0, 25);

    let min_distance = dijkstra(&height_map, start, end, width, height);

    println!("{}", min_distance);
}

fn part_2() {
    let input = include_str!("../input");
    let mut height_map: Vec<_> = input
        .bytes()
        .filter(|b| b != &b'\n')
        .map(|b| b.to_ascii_lowercase() - b'a')
        .collect();
    
    let width = input.bytes().position(|b| b == b'\n').unwrap();
    let height = height_map.len() / width;

    let mut start = input.bytes().position(|b| b == b'S').unwrap();
    let mut end = input.bytes().position(|b| b == b'E').unwrap();

    // Change coordinate to height map coordinate and update values:
    (start, end) = (start - start / (width + 1), end - end / (width + 1));
    (height_map[start], height_map[end]) = (0, 25);

    let distances = height_map
        .iter()
        .enumerate()
        .filter(|(_, h)| **h == 0)
        .filter_map(|(pos, _)| {
            let distance = dijkstra(&height_map, pos, end, width, height);
            match distance {
                0 => None,
                v => Some(v)
            }
        });

    println!("{}", distances.min().unwrap());
}

fn main() {
    part_1();
    part_2();
}
