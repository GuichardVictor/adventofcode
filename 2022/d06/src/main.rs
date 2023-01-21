use std::collections::HashSet;

fn part_1() {
    let input = include_str!("../input");
    let pos = input
        .as_bytes()
        .windows(4)
        .position(|x| {
            x[0] != x[1]
                && x[0] != x[2]
                && x[0] != x[3]
                && x[1] != x[2]
                && x[1] != x[3]
                && x[2] != x[3]
        })
        .unwrap();

    println!("{}", pos + 4);
}

fn part_2() {
    let input = include_str!("../input");

    let pos = input
        .as_bytes()
        .windows(14)
        .position(|x| {
            let set: HashSet<_> = x.iter().collect();
            set.len() == 14
        })
        .unwrap();

    println!("{}", pos + 14);
}

fn main() {
    part_1();
    part_2();
}
