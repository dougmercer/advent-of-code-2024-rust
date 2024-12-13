use advent_2024::digits;
use itertools::Itertools;
use std::collections::HashMap;
use std::{error::Error, fs};

fn parse_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn apply_rule(rock: u64) -> Vec<u64> {
    match rock {
        0 => vec![1],
        r if digits(r.into()) % 2 == 0 => {
            let mid = digits(r.into()) / 2;
            let s = r.to_string();
            vec![
                s[..mid as usize].parse().unwrap(),
                s[mid as usize..].parse().unwrap(),
            ]
        }
        r => vec![r * 2024],
    }
}

fn problem(input: &str, iterations: usize) -> usize {
    let mut rocks = parse_input(input).into_iter().counts();

    for _ in 0..iterations {
        rocks = rocks
            .into_iter()
            .flat_map(|(rock, count)| {
                apply_rule(rock)
                    .into_iter()
                    .map(move |new_rock| (new_rock, count))
            })
            .fold(HashMap::new(), |mut acc, (rock, count)| {
                *acc.entry(rock).or_default() += count;
                acc
            });
    }

    rocks.values().sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "data/day11.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:}", problem(&input, 25));
    println!("Part 2: {:}", problem(&input, 75));
    Ok(())
}

#[test]
fn test_1() {
    assert_eq!(problem(&"125 17", 1), 3);
}

#[test]
fn test_2() {
    assert_eq!(problem(&"125 17", 2), 4);
}

#[test]
fn test_3() {
    assert_eq!(problem(&"125 17", 3), 5);
}

#[test]
fn test_4() {
    assert_eq!(problem(&"125 17", 4), 9);
}

#[test]
fn test_5() {
    assert_eq!(problem(&"125 17", 5), 13);
}

#[test]
fn test_6() {
    assert_eq!(problem(&"125 17", 6), 22);
}

#[test]
fn test_25() {
    assert_eq!(problem(&"125 17", 25), 55312);
}
