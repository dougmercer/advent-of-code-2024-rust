use std::{error::Error, fs};

use itertools::Itertools;

#[derive(Debug)]
struct Block {
    id: String,
    start: usize,
    width: usize,
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .trim()
        .split("")
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

fn decode(encoded: &[String]) -> Vec<String> {
    encoded
        .iter()
        .enumerate()
        .map(|(k, num_str)| {
            let n = num_str.parse::<usize>().unwrap();
            match k % 2 {
                0 => vec![(k / 2).to_string(); n],
                _ => vec!['.'.to_string(); n],
            }
        })
        .flatten()
        .collect()
}

fn compress(data: &mut Vec<String>) -> &mut Vec<String> {
    let mut left: usize = 0;
    let mut right: usize = data.len().saturating_sub(1);
    while left < right {
        while right > left && data[right] == "." {
            right = right.saturating_sub(1);
        }

        while left < right && data[left] != "." {
            left += 1;
        }

        if left < right {
            data.swap(left, right);
            left += 1;
            right = right.saturating_sub(1);
        }
    }

    data
}

fn compress2(data: &[String]) -> Vec<String> {
    let mut free_blocks: Vec<Block> = Vec::new();
    let mut claimed_blocks: Vec<Block> = Vec::new();

    let mut k = 0;
    while k < data.len() {
        let mut block_width = 0;
        let block_id = &data[k];

        // Find the block width
        while (k + block_width) < data.len() && data[k + block_width] == *block_id {
            block_width += 1;
        }

        let block = Block {
            id: block_id.to_string(),
            start: k,
            width: block_width,
        };
        if block_id == "." {
            free_blocks.push(block);
        } else {
            claimed_blocks.push(block);
        }

        k += block_width;
    }

    claimed_blocks.sort_by_key(|b| std::cmp::Reverse(b.id.parse::<usize>().unwrap()));

    for block in claimed_blocks.iter_mut() {
        for (k, maybe) in free_blocks.iter_mut().enumerate() {
            if block.width <= maybe.width && block.start > maybe.start {
                let new_free_block = Block {
                    id: ".".to_string(),
                    start: block.start,
                    width: block.width,
                };
                block.start = maybe.start;
                maybe.start += block.width;
                maybe.width -= block.width;
                if maybe.width == 0 {
                    free_blocks.remove(k);
                }
                free_blocks.push(new_free_block);

                break;
            }
        }
    }

    free_blocks
        .iter()
        .chain(claimed_blocks.iter())
        .sorted_by_key(|x| x.start)
        .map(|x| vec![x.id.clone(); x.width])
        .flatten()
        .collect()
}

fn checksum(compressed: &[String]) -> usize {
    compressed
        .iter()
        .enumerate()
        .filter(|(_, b)| *b != ".")
        .map(|(k, data)| k * data.parse::<usize>().unwrap())
        .sum()
}

fn part1(input: &str) -> usize {
    let encoded = parse_input(&input);
    let mut data = decode(&encoded);
    let compressed = compress(&mut data);
    checksum(compressed)
}

fn part2(input: &str) -> usize {
    let encoded = parse_input(&input);
    let data = decode(&encoded);
    let compressed = compress2(&data);
    checksum(&compressed)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "data/day9.input";
    let input = fs::read_to_string(path)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[test]
fn test_part1() {
    let input = "2333133121414131402";
    assert_eq!(part1(&input), 1928);
}

#[test]
fn test_part2() {
    let input = "2333133121414131402";
    assert_eq!(part2(&input), 2858);
}
