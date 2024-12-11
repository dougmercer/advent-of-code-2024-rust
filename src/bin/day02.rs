use std::{error::Error, fs};

fn read_input(path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let result: Result<Vec<Vec<i32>>, Box<dyn Error>> = fs::read_to_string(path)?
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| {
                    s.parse()
                        .map_err(|e| format!("Invalid input: {}", e).into())
                })
                .collect()
        })
        .collect();
    Ok(result?)
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

fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    Ok(read_input(path)?
        .iter()
        .filter(|&report| is_safe(report))
        .count())
}

fn part2(path: &str) -> Result<usize, Box<dyn Error>> {
    Ok(read_input(path)?
        .iter()
        .filter(|&report| problem_dampener(report))
        .count())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day2.input";
    println!("Part 1: {:?}", part1(path)?);
    println!("Part 2: {:?}", part2(path)?);
    Ok(())
}
