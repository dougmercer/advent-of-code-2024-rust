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

fn part1(path: &str) -> Result<i32, Box<dyn Error>> {
    let reports = read_input(path)?;

    fn is_safe(report: &Vec<i32>) -> bool {
        let monotonic: bool =
            report.windows(2).all(|w| w[0] <= w[1]) || report.windows(2).all(|w| w[0] >= w[1]);
        let valid_diffs = report.windows(2).all(|w| {
            let diff = (w[0] - w[1]).abs();
            diff >= 1 && diff <= 3
        });
        monotonic && valid_diffs
    }

    Ok(reports.iter().filter(|&report| is_safe(report)).count() as i32)
}

fn main() -> Result<(), Box<dyn Error>> {
    // let path: &str = "data/day2.sample.csv";
    let path: &str = "data/day2.input";
    println!("Part 1: {:?}", part1(path)?);
    // println!("Part 2: {:?}", part2(path)?);
    Ok(())
}
