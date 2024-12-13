use itertools::repeat_n;
use itertools::Itertools;
use rayon::prelude::*;
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

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
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
        .collect()
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

fn part(path: &str, ops: &[Operator]) -> u64 {
    parse_input(path)
        .par_iter()
        .filter(|(result, values)| find_answer(result, values, &ops))
        .map(|(a, _)| a)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "data/day7.input";
    let input = fs::read_to_string(path)?;
    let ops_part1 = vec![Operator::Add, Operator::Multiply];
    println!("Part 1: {:?}", part(&input, &ops_part1));

    let ops_part2 = vec![Operator::Add, Operator::Multiply, Operator::Concat];
    println!("Part 2: {:?}", part(&input, &ops_part2));
    Ok(())
}

#[test]
fn test_part1() {
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    let ops_part1 = vec![Operator::Add, Operator::Multiply];
    assert_eq!(part(&input, &ops_part1), 3749);
}

#[test]
fn test_part2() {
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    let ops_part2 = vec![Operator::Add, Operator::Multiply, Operator::Concat];

    assert_eq!(part(&input, &ops_part2), 11387);
}
