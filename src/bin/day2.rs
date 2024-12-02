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

// fn count(values: &[i32]) -> HashMap<i32, i32> {
//     values.iter().fold(HashMap::new(), |mut map, &x| {
//         *map.entry(x).or_insert(0) += 1;
//         map
//     })
//     // // Original approach
//     // let mut map = HashMap::new();
//     // for &x in values.iter() {
//     //     *map.entry(x).or_insert(0) += 1;
//     // }
//     // map
// }

// fn part1(path: &str) -> Result<i32, Box<dyn Error>> {
//     let (col1, col2) = read_input(path)?;

//     // // Original Approach
//     // let mut distance: i32 = 0;
//     // for (&val1, &val2) in col1.iter().zip(col2.iter()) {
//     //     distance += (val1 - val2).abs();
//     // }

//     Ok(col1
//         .iter()
//         .zip(col2.iter())
//         .map(|(a, b)| (a - b).abs())
//         .sum())
// }

// fn part2(path: &str) -> Result<i32, Box<dyn Error>> {
//     let (col1, col2) = read_input(path)?;

//     // Compute similarity
//     let counter1 = count(&col1);
//     let counter2 = count(&col2);

//     // // Original Approach
//     // let counter1 = count(&col1);
//     // let counter2 = count(&col2);
//     // let mut similarity: i32 = 0;
//     // for (&key, &val1) in counter1.iter() {
//     //     if let Some(&val2) = counter2.get(&key) {
//     //         similarity += key * val1 * val2;
//     //     }
//     // }

//     Ok(counter1
//         .iter()
//         .filter_map(|(&key, &val1)| counter2.get(&key).map(|val2| key * val1 * val2))
//         .sum())
// }

fn part1(path: &str) -> Result<i32, Box<dyn Error>> {
    let reports = read_input(path)?;

    // let mut n_safe: i32 = 0;
    // let mut ascending: bool;
    // let mut descending: bool;
    // let mut diff_small: bool;

    // for report in reports.iter() {
    //     ascending = report.windows(2).all(|w| w[0] <= w[1]);
    //     descending = report.windows(2).all(|w| w[0] >= w[1]);
    //     diff_small = report.windows(2).all(|w| (w[0] - w[1]).abs() >=1 && (w[0] - w[1]).abs() <=3);
    //     n_safe += ((ascending || descending) && diff_small) as i32;
    // }

    // for report in reports.iter() {
    //     let monotonic =
    //         report.windows(2).all(|w| w[0] <= w[1]) || report.windows(2).all(|w| w[0] >= w[1]);
    //     let valid_diffs = report.windows(2).all(|w| {
    //         let diff = (w[0] - w[1]).abs();
    //         diff >= 1 && diff <= 3
    //     });
    //     n_safe += (monotonic && valid_diffs) as i32;
    // }

    // for report in reports.iter() {
    //     let monotonic =
    //         report.windows(2).all(|w| w[0] <= w[1]) || report.windows(2).all(|w| w[0] >= w[1]);
    //     let valid_diffs = report.windows(2).all(|w| {
    //         let diff = (w[0] - w[1]).abs();
    //         diff >= 1 && diff <= 3
    //     });
    //     n_safe += (monotonic && valid_diffs) as i32;
    // }

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
