fn part_1() {
    let input = include_str!("../input");

    let result = input.split("\n")
        .map(|line| line.split_once(",").unwrap())
        .map(|(assignment, assigned)| {
            let (l1, u1) = assignment.split_once("-").unwrap();
            let (l2, u2) = assigned.split_once("-").unwrap();

            (
                l1.parse::<u8>().unwrap(),
                u1.parse::<u8>().unwrap(),
                l2.parse::<u8>().unwrap(),
                u2.parse::<u8>().unwrap(),
            )
        })
        .filter(|(l1, u1, l2, u2)| (l1 >= l2 && u1 <= u2) || (l1 <= l2 && u1 >= u2))
        .count();

    println!("{}", result);
}

fn part_2() {
    let input = include_str!("../input");

    let result = input.split("\n")
        .map(|line| line.split_once(",").unwrap())
        .map(|(assignment, assigned)| {
            let (l1, u1) = assignment.split_once("-").unwrap();
            let (l2, u2) = assigned.split_once("-").unwrap();

            (
                l1.parse::<u8>().unwrap(),
                u1.parse::<u8>().unwrap(),
                l2.parse::<u8>().unwrap(),
                u2.parse::<u8>().unwrap(),
            )
        })
        .filter(|(l1, u1, l2, u2)| (l1 <= u2) && (l2 <= u1))
        .count();

    println!("{}", result);
}

fn main() {
    part_1();
    part_2();
}
