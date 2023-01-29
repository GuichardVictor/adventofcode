use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Packet {
    Number(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => match a.iter().cmp(b) {
                r if r != Ordering::Equal => r,
                _ => a.len().cmp(&b.len()),
            },
            (Packet::Number(_), Packet::List(b)) if b.len() == 1 => self.cmp(&b[0]),
            (Packet::Number(a), Packet::List(_)) => {
                Packet::List(vec![Packet::Number(*a)]).cmp(other)
            }
            (Packet::List(_), Packet::Number(_)) => other.cmp(self).reverse(),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_list(input: &mut Peekable<Chars>) -> Packet {
    let mut items = vec![];
    while let Some(c) = input.peek() {
        match c {
            ']' => {
                input.next();
                break;
            }
            ',' => {
                input.next();
                items.push(parse_packet(input))
            }
            _ => items.push(parse_packet(input)),
        }
    }

    Packet::List(items)
}

fn parse_number(input: &mut Peekable<Chars>, c: char) -> Packet {
    let mut num = c.to_string();

    while let Some(c) = input.peek() {
        match c {
            ',' | ']' => break,
            v => {
                num.push(v.clone());
                input.next();
            }
        }
    }

    Packet::Number(num.parse().unwrap())
}

fn parse_packet(input: &mut Peekable<Chars>) -> Packet {
    match input.next() {
        Some('[') => parse_list(input),
        Some(c) => parse_number(input, c),
        None => Packet::List(vec![]),
    }
}

fn part_1() {
    let input = include_str!("../input");
    let total: usize = input
        .split("\n\n")
        .map(|pair| pair.split_once("\n").unwrap())
        .map(|(a, b)| {
            (
                parse_packet(&mut a.chars().peekable()),
                parse_packet(&mut b.chars().peekable()),
            )
        })
        .enumerate()
        .filter(|(_, (a, b))| a.cmp(b) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();

    println!("{}", total);
}

fn part_2() {
    let input = include_str!("../input");

    let d1 = Packet::List(vec![Packet::List(vec![Packet::Number(2)])]);
    let d2 = Packet::List(vec![Packet::List(vec![Packet::Number(6)])]);
    let packets: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_packet(&mut l.chars().peekable()))
        .filter(|packet| packet < &d2)
        .collect();

    let result = (packets.iter().filter(|i| *i < &d1).count() + 1) * (packets.len() + 2);

    println!("{}", result)
}

fn main() {
    part_1();
    part_2();
}
