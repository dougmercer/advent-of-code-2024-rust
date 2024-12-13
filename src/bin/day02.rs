use std::{error::Error, fs};

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    let monotonic: bool =
        report.windows(2).all(|w| w[0] <= w[1]) || report.windows(2).all(|w| w[0] >= w[1]);
    let valid_diffs = report.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        diff >= 1 && diff <= 3
    });
    monotonic && valid_diffs
}

fn hold_out(x: &[i32], i: usize) -> Vec<i32> {
    x.iter()
        .take(i)
        .chain(x.iter().skip(i + 1))
        .copied()
        .collect()
}

fn problem_dampener(report: &[i32]) -> bool {
    (0..report.len()).any(|i| is_safe(&hold_out(report, i)))
}

fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|&report| is_safe(report))
        .count()
}

fn part2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|&report| problem_dampener(report))
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day2.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
    Ok(())
}

#[test]
fn test_part1() {
    let input = [
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ]
    .join("\n");
    assert_eq!(part1(&input), 2);
}

#[test]
fn test_part2() {
    let input = [
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ]
    .join("\n");
    assert_eq!(part2(&input), 4);
}
