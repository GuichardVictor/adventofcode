fn part_1() {
    let input = include_bytes!("../input");
    let score = input.split(|b|*b == b'\n')
        .map(|line| ((line[0] - b'A') as i16, (line[2] - b'X') as i16,))
        .map(|(opponent, mine)| 1 + mine + 3 * (1 + mine - opponent).rem_euclid(3))
        .sum::<i16>();

    println!("{}", score)
}

fn part_2() {
    let input = include_bytes!("../input");
    let score = input.split(|b|*b == b'\n')
        .map(|line| ((line[0] - b'A') as i16, (line[2] - b'X') as i16,))
        .map(|(opponent, mine)| 3 * mine + 1 + (2 + mine + opponent).rem_euclid(3))
        .sum::<i16>();

    println!("{}", score)
}

fn main() {
    part_1();
    part_2();
}
