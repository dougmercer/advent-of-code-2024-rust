use itertools::Itertools;
use std::{error::Error, fs};

const CALIBRATION_VALUE: f64 = 10000000000000.0;

#[derive(Debug)]
struct Problem {
    button_a: (f64, f64),
    button_b: (f64, f64),
    prize: (f64, f64),
}

fn parse_numbers(line: &str) -> Option<(u32, u32)> {
    let nums: Vec<u32> = line
        .split(&['+', '=', ','])
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    match nums[..] {
        [x, y] => Some((x, y)),
        _ => None,
    }
}

fn parse_input(input: &str, calibrate: bool) -> Vec<Problem> {
    let calibration = if calibrate { CALIBRATION_VALUE } else { 0.0 };

    input
        .trim()
        .split("\n\n")
        .filter_map(|game| {
            let (a, b, prize) = game.lines().filter_map(parse_numbers).collect_tuple()?;

            Some(Problem {
                button_a: (a.0 as f64, a.1 as f64),
                button_b: (b.0 as f64, b.1 as f64),
                prize: (prize.0 as f64 + calibration, prize.1 as f64 + calibration),
            })
        })
        .collect()
}

fn is_integer(x: f64) -> bool {
    (x.round() - x).abs() < 1e-16
}

fn solve_problem(problem: &Problem) -> Option<(u64, u64)> {
    let a = problem.button_a.0;
    let b = problem.button_b.0;
    let c = problem.button_a.1;
    let d = problem.button_b.1;
    let det = a * d - b * c;
    let price = (problem.prize.0, problem.prize.1);

    if det == 0.0 {
        return None;
    }

    let a_presses = (d * price.0 - b * price.1) / det;
    let b_presses = (-c * price.0 + a * price.1) / det;

    if !is_integer(a_presses) || !is_integer(b_presses) {
        return None;
    }

    Some((a_presses as u64, b_presses as u64))
}

fn solver(input: &str, calibrate: bool) -> u64 {
    parse_input(input, calibrate)
        .iter()
        .filter_map(solve_problem)
        .map(|(x, y)| 3 * x + y)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day13.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:?}", solver(&input, false));
    println!("Part 2: {:?}", solver(&input, true));
    Ok(())
}

#[test]
fn test_part1() {
    let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    assert_eq!(solver(&input, false), 480);
}

#[test]
fn test_part2() {
    let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

    // AoC doesn't give answer for this...
    assert_eq!(solver(&input, true), 875318608908);
}
