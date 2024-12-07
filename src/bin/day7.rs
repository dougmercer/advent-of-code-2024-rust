use itertools::repeat_n;
use itertools::Itertools;
use std::{error::Error, fs, iter::successors};

// https://stackoverflow.com/a/69302957
// Key idea-- then() returns an Option, so this ends when the value is smaller than 10.
fn digits(n: u64) -> u32 {
    successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count() as u32
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => a * u64::pow(10, digits(b)) + b,
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => '+',
                Operator::Multiply => '*',
                Operator::Concat => '|',
            }
        )
    }
}

fn read_input(path: &str) -> Result<Vec<(u64, Vec<u64>)>, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .map(|line| {
            let (a, rest) = line.split_once(':').unwrap();
            (
                a.parse().unwrap(),
                rest.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
        })
        .collect())
}

fn find_answer(result: &u64, values: &[u64], ops: &[Operator]) -> bool {
    let first = values[0];
    let rest: &[u64] = &values[1..];

    repeat_n(ops.iter(), rest.len())
        .multi_cartesian_product()
        .any(|ops| {
            rest.iter()
                .zip(ops)
                .fold(first, |acc, (&val, &op)| op.apply(acc, val))
                == *result
        })
}

fn part(path: &str, ops: &[Operator]) -> Result<u64, Box<dyn Error>> {
    Ok(read_input(path)?
        .iter()
        .filter(|(result, values)| find_answer(result, values, &ops))
        .map(|(a, _)| a)
        .sum())
}

fn main() {
    let path = "data/day7.input";

    let ops_part1 = vec![Operator::Add, Operator::Multiply];
    println!("{:?}", part(path, &ops_part1).unwrap());

    let ops_part2 = vec![Operator::Add, Operator::Multiply, Operator::Concat];
    println!("{:?}", part(path, &ops_part2).unwrap());
}
