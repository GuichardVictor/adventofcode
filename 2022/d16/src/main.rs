use std::{collections::HashMap, error::Error, str::FromStr};

#[derive(Debug)]
struct Valve {
    pub flow_rate: usize,
    pub neighbords: Vec<String>,
    pub name: String,
}

impl FromStr for Valve {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iterator = s.split(" ");
        iterator.next().unwrap();

        let name = iterator.next().unwrap().to_string();
        iterator.next().unwrap();
        iterator.next().unwrap();
        let flow_rate = iterator
            .next()
            .unwrap()
            .split_once("=")
            .unwrap()
            .1
            .replace(";", "")
            .parse()
            .unwrap();

        let neighbors = iterator
            .skip_while(|substring| !substring.contains("valve"))
            .skip(1)
            .map(|e| e.replace(",", ""))
            .collect();

        Ok(Valve {
            flow_rate: flow_rate,
            name: name,
            neighbords: neighbors,
        })
    }
}

fn part_1() {
    let input = include_str!("../input");
    let valves: HashMap<String, Valve> = input
        .split("\n")
        .map(|line| Valve::from_str(line))
        .map(|v| v.unwrap())
        .map(|v| (v.name.clone(), v))
        .collect();

    println!("{:?}", valves);
}

fn main() {
    part_1();
}
