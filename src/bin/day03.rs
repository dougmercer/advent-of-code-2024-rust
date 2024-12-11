use atoi::atoi;
use std::{error::Error, fs};

fn read_input(path: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}

enum Command {
    Mul(i32),
    Do,
    Dont,
}

// Parse a number of up to 3 digits, returning (number, chars consumed)
fn parse_number(bytes: &[u8]) -> Option<(i32, usize)> {
    let len = bytes
        .iter()
        .take_while(|&&b| b.is_ascii_digit())
        .take(3)
        .count();

    if len == 0 {
        return None;
    }

    Some((atoi(&bytes[..len])?, len))
}

fn try_parse_mul(bytes: &[u8]) -> Option<(Command, usize)> {
    if bytes.starts_with(b"mul(") {
        let (a, consumed) = parse_number(&bytes[4..])?;
        let next_pos = 4 + consumed;

        if bytes.get(next_pos) != Some(&b',') {
            return None;
        }

        let (b, consumed) = parse_number(&bytes[next_pos + 1..])?;
        let next_pos = next_pos + 1 + consumed;

        if bytes.get(next_pos) != Some(&b')') {
            return None;
        }

        return Some((Command::Mul(a * b), next_pos + 1));
    }

    None
}

fn try_parse_command(bytes: &[u8]) -> Option<(Command, usize)> {
    // Try to parse "do()"
    if bytes.starts_with(b"do()") {
        return Some((Command::Do, 4));
    }

    // Try to parse "don't()"
    if bytes.starts_with(b"don't()") {
        return Some((Command::Dont, 7));
    }

    try_parse_mul(bytes)
}

fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let content = read_input(path)?;
    let bytes = content.as_bytes();

    let sum: i32 = (0..bytes.len())
        .filter_map(|i| try_parse_command(&bytes[i..]))
        .fold(0, |sum, (command, _)| match command {
            Command::Mul(result) => sum + result,
            _ => sum,
        });

    Ok(sum.try_into().unwrap())
}

fn part2(path: &str) -> Result<usize, Box<dyn Error>> {
    let content = read_input(path)?;
    let bytes = content.as_bytes();

    let mut enabled = true;
    let sum: i32 = (0..bytes.len())
        .filter_map(|i| try_parse_command(&bytes[i..]))
        .fold(0, |sum, (command, _)| match command {
            Command::Mul(result) if enabled => sum + result,
            Command::Do => {
                enabled = true;
                sum
            }
            Command::Dont => {
                enabled = false;
                sum
            }
            _ => sum,
        });

    Ok(sum.try_into().unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day3.input";
    println!("Part 1: {:?}", part1(path)?);
    println!("Part 2: {:?}", part2(path)?);
    Ok(())
}
