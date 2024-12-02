use std::{error::Error, fs};

fn read_input(path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in content.lines() {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        reports.push(values);
    }

    Ok(reports)
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
    .chain(x.iter().skip(i+1))
    .copied()
    .collect()
}

fn problem_dampener(report: &[i32]) -> bool {
    let mut _is_safe = false;
    for i in 0..report.len() {
        let report_without_i = hold_out(report, i);
        _is_safe = _is_safe || is_safe(&report_without_i);
    }
    _is_safe
}

fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let reports: Vec<Vec<i32>> = read_input(path)?;
    Ok(reports.iter().filter(|&report| is_safe(report)).count())
}

fn part2(path: &str) -> Result<usize, Box<dyn Error>> {
    let reports: Vec<Vec<i32>> = read_input(path)?;
    Ok(reports.iter().filter(|&report| problem_dampener(report)).count())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day2.input";
    println!("Part 1: {:?}", part1(path)?);
    println!("Part 2: {:?}", part2(path)?);
    Ok(())
}
