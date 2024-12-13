use atoi::atoi;
use std::{error::Error, fs};

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

fn part1(input: &str) -> usize {
    let bytes = input.as_bytes();

    let sum: i32 = (0..bytes.len())
        .filter_map(|i| try_parse_command(&bytes[i..]))
        .fold(0, |sum, (command, _)| match command {
            Command::Mul(result) => sum + result,
            _ => sum,
        });

    sum.try_into().unwrap()
}

fn part2(input: &str) -> usize {
    let bytes = input.as_bytes();

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

    sum.try_into().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day3.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
    Ok(())
}

#[test]
fn test_part1() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part1(&input), 161);
}

#[test]
fn test_part2() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2(&input), 48);
}
