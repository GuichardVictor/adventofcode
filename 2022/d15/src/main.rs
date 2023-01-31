use std::ops::RangeInclusive;

const ROW: isize = 2_000_000;

fn distance(p1: (isize, isize), p2: (isize, isize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn merge_range(ranges: &mut Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
    let mut merged_ranges = vec![];
    let mut merging_range: RangeInclusive<isize> = ranges.pop().unwrap();

    loop {
        let merge_index = ranges.iter().position(|other| {
            merging_range.contains(other.start()) || merging_range.contains(other.end())
        });
        merging_range = match merge_index {
            Some(pos) => {
                let other = ranges.remove(pos);
                *(merging_range.start().min(other.start()))
                    ..=*(merging_range.end().max(other.end()))
            }
            None => {
                merged_ranges.push(merging_range);
                match ranges.pop() {
                    Some(range) => range,
                    None => break,
                }
            }
        }
    }

    merged_ranges
}

fn part_1() {
    let input = include_str!("../input");

    let mut ranges: Vec<_> = input
        .split("\n")
        .map(|line| {
            let coordinates: Vec<_> = line
                .split(" ")
                .filter_map(|word| {
                    if word.starts_with("x=") {
                        let (_, number) = word.split_once("x=").unwrap();
                        return number[..number.len() - 1].parse::<isize>().ok();
                    }

                    if word.starts_with("y=") {
                        let (_, number) = word.split_once("y=").unwrap();
                        if number.ends_with(":") {
                            return number[..number.len() - 1].parse::<isize>().ok();
                        }
                        return number.parse::<isize>().ok();
                    }

                    None
                })
                .collect();

            let (sensor, beacon) = (
                (coordinates[0], coordinates[1]),
                (coordinates[2], coordinates[3]),
            );
            let distance = distance(sensor, beacon);

            (sensor, beacon, distance)
        })
        .filter(|(sensor, _, distance)| sensor.1.abs_diff(ROW) <= *distance)
        .map(|(sensor, _, distance)| {
            let radius = distance as isize - sensor.1.abs_diff(ROW) as isize;

            sensor.0 - radius..=sensor.0 + radius
        })
        .collect();

    let merged_ranges = merge_range(&mut ranges);

    let count: isize = merged_ranges
        .iter()
        .map(|range| range.end() - range.start())
        .sum();

    println!("{}", count);
}

fn main() {
    part_1();
}
