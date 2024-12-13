use std::collections::HashMap;
use std::{error::Error, fs};

fn read_input(path: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in content.lines() {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        col1.push(values[0]);
        col2.push(values[1]);
    }

    col1.sort();
    col2.sort();

    Ok((col1, col2))
}

fn count(values: &[i32]) -> HashMap<i32, i32> {
    values.iter().fold(HashMap::new(), |mut map, &x| {
        *map.entry(x).or_insert(0) += 1;
        map
    })
    // // Original approach
    // let mut map = HashMap::new();
    // for &x in values.iter() {
    //     *map.entry(x).or_insert(0) += 1;
    // }
    // map
}

fn part1(path: &str) -> Result<i32, Box<dyn Error>> {
    let (col1, col2) = read_input(path)?;

    // // Original Approach
    // let mut distance: i32 = 0;
    // for (&val1, &val2) in col1.iter().zip(col2.iter()) {
    //     distance += (val1 - val2).abs();
    // }

    Ok(col1
        .iter()
        .zip(col2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum())
}

fn part2(path: &str) -> Result<i32, Box<dyn Error>> {
    let (col1, col2) = read_input(path)?;

    // Compute similarity
    let counter1 = count(&col1);
    let counter2 = count(&col2);

    // // Original Approach
    // let counter1 = count(&col1);
    // let counter2 = count(&col2);
    // let mut similarity: i32 = 0;
    // for (&key, &val1) in counter1.iter() {
    //     if let Some(&val2) = counter2.get(&key) {
    //         similarity += key * val1 * val2;
    //     }
    // }

    Ok(counter1
        .iter()
        .filter_map(|(&key, &val1)| counter2.get(&key).map(|val2| key * val1 * val2))
        .sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    // let path: &str = "data/day1.sample";
    let path: &str = "data/day1.input";
    println!("Part 1: {:?}", part1(path)?);
    println!("Part 2: {:?}", part2(path)?);
    Ok(())
}
