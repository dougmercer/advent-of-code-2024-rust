// Didn't really have time to do today's challenge cause I was on travel =/
use std::{error::Error, fs};

fn read_input(path: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?)
}

fn part1(path: &str) -> Result<usize, Box<dyn Error>> {
    let code = read_input(path)?;
    let chars: Vec<char> = code.chars().collect();
    let mut i: usize = 0;
    let mut j: usize;
    let mut a: i32;
    let mut b: i32;
    let mut answer: i32 = 0;
    while i < chars.len() {
        if chars[i] != 'm' {
            i += 1;
            continue;
        }
        i += 1;
        if chars[i] != 'u' {
            i += 1;
            continue;
        }
        i += 1;
        if chars[i] != 'l' {
            i += 1;
            continue;
        }
        i += 1;
        if chars[i] != '(' {
            i += 1;
            continue;
        }
        i += 1;
        j = 0;
        while (i + j) < chars.len() && chars[i + j].is_digit(10) {
            j += 1
        }
        if j > 3 {
            i += j;
            continue;
        } else {
            let num_str_a: String = chars[i..i + j].iter().collect();
            a = num_str_a.parse::<i32>().unwrap();
        }
        i += j;

        if chars[i] != ',' {
            i += 1;
            continue;
        }
        i += 1;
        j = 0;
        while (i + j) < chars.len() && chars[i + j].is_digit(10) {
            j += 1
        }
        if j > 3 {
            i += j;
            continue;
        } else {
            let num_str_b: String = chars[i..i + j].iter().collect();
            b = num_str_b.parse::<i32>().unwrap();
        }
        i += j;
        if chars[i] != ')' {
            i += 1;
            continue;
        }
        i += 1;
        // println!("a={} b={}", a, b);
        answer += a * b;
    }

    Ok(answer.try_into().unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day3.input";
    println!("Part 1: {:?}", part1(path)?);
    Ok(())
}
