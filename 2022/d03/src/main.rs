
fn part_1() {
    let input = include_bytes!("../input");

    let result : i16 = input.split(|b| *b == b'\n')
        .map(|line| line.split_at(line.len() / 2))
        .map(|(w1, w2)| w1.iter().find(|b| w2.contains(b)).unwrap())
        .map(|b| 1 + (b.to_ascii_lowercase() - b'a') as i16 + 26 * b.is_ascii_uppercase() as i16)
        .sum();

    println!("{}", result);
}

fn part_2() {
    let input = include_bytes!("../input");

    let result : i16 = input.split(|b| *b == b'\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|set| set[0]
            .iter()
            .find(|b| set[1].contains(b) && set[2].contains(b))
            .unwrap())
        .map(|b| 1 + (b.to_ascii_lowercase() - b'a') as i16 + 26 * b.is_ascii_uppercase() as i16)
        .sum();

    println!("{}", result);
}

fn main() {
    part_1();
    part_2();
}
