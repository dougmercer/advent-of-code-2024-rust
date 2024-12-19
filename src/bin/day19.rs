use itertools::Itertools;
use std::{error::Error, fs};

fn parse_input(input: &str) -> Result<(Vec<&str>, Vec<&str>), Box<dyn Error>> {
    let parts = input.split_once("\n\n");
    let patterns = parts
        .ok_or("No patterns found")?
        .0
        .split(", ")
        .sorted()
        .collect();
    let designs = parts
        .ok_or("No designs found")?
        .1
        .lines()
        .sorted()
        .collect();

    Ok((patterns, designs))
}

fn count_ways(design: &str, patterns: &[&str]) -> usize {
    let n = design.len();
    let mut dp: Vec<usize> = vec![0; n + 1];
    dp[0] = 1; // One way to make empty string

    for i in 0..n {
        // Skip if no ways to reach this position
        if dp[i] == 0 {
            continue;
        }

        // Try each pattern
        for &pattern in patterns {
            if design[i..].starts_with(pattern) {
                let new_pos = i + pattern.len();

                // Add number of ways to reach current position
                dp[new_pos] += dp[i];
            }
        }
    }

    dp[n]
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let (patterns, designs) = parse_input(input)?;
    Ok(designs
        .iter()
        .filter(|design| count_ways(design, &patterns) > 0)
        .count())
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let (patterns, designs) = parse_input(input)?;
    Ok(designs
        .iter()
        .map(|design| count_ways(design, &patterns))
        .sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day19.input")?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&EXAMPLE).unwrap(), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&EXAMPLE).unwrap(), 16);
    }
}
