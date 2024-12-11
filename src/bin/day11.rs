use advent_2024::digits;
use std::collections::HashMap;
use itertools::Itertools;
use std::{error::Error, fs};

fn read_input(path: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect())
}

fn apply_rule(rock: u64) -> Vec<u64> {
    match rock {
        0 => vec![1],
        r if digits(r.into()) % 2 == 0 => {
            let mid = digits(r.into()) / 2;
            let s = r.to_string();
            vec![
                s[..mid as usize].parse().unwrap(),
                s[mid as usize..].parse().unwrap()
            ]
        }
        r => vec![r * 2024]
    }
}

fn problem(path: &str, iterations: usize) -> usize {
    let mut rocks = read_input(path).unwrap().into_iter().counts();

    for _ in 0..iterations {
        rocks = rocks
            .into_iter()
            .flat_map(|(rock, count)| {
                apply_rule(rock).into_iter().map(move |new_rock| (new_rock, count))
            })
            .fold(HashMap::new(), |mut acc, (rock, count)| {
                *acc.entry(rock).or_default() += count;
                acc
            });
    }

    rocks.values().sum()
}

fn main() {
    // let path = "data/day11.sample";
    let path = "data/day11.input";
    println!("Part 1: {:?} rocks", problem(&path, 25));
    println!("Part 2: {:?} rocks", problem(&path, 75));
}
