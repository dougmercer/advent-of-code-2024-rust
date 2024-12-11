use itertools::Itertools;
use std::{error::Error, fs};
use topological_sort::TopologicalSort;

fn read_input(path: &str) -> Result<(Vec<(u32, u32)>, Vec<Vec<u32>>), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    // Split content into two parts on double newline
    let (rules_str, pages_str) = content.split_once("\n\n").unwrap_or((&content, ""));

    // Parse the rules
    let rules = rules_str
        .lines()
        .map(|line| {
            line.split('|')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    // Parse the page orders
    let pages = pages_str
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    Ok((rules, pages))
}

fn is_relevant_rule(rule: (u32, u32), pages: &[u32]) -> bool {
    pages.contains(&rule.0) && pages.contains(&rule.1)
}

fn sort_by_rules(rules: Vec<(u32, u32)>, pages: Vec<u32>) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut ts = TopologicalSort::<u32>::new();

    // Insert all pages first
    for page in &pages {
        ts.insert(*page);
    }

    // Add dependencies from relevant rules
    for (before, after) in rules.iter().filter(|rule| is_relevant_rule(**rule, &pages)) {
        ts.add_dependency(*before, *after);
    }

    // Build result vector by popping in sorted order
    let mut result = Vec::with_capacity(pages.len());
    while !ts.is_empty() {
        let batch = ts.pop_all();
        if batch.is_empty() {
            return Err("Cycle detected in dependencies".into());
        }
        let batch_vec: Vec<_> = batch.into_iter().collect();
        result.extend(batch_vec);
    }

    Ok(result)
}

fn get_midpoint(values: &[u32]) -> u32 {
    values[values.len() / 2]
}

fn part1(path: &str) -> Result<u32, Box<dyn Error>> {
    let (rules, orders) = read_input(path).unwrap();
    let total: u32 = orders
        .into_iter()
        .filter_map(|original_order| {
            let sorted_order = sort_by_rules(rules.clone(), original_order.clone()).unwrap();
            if original_order == sorted_order {
                Some(get_midpoint(&sorted_order))
            } else {
                None
            }
        })
        .sum();
    Ok(total)
}

fn part2(path: &str) -> Result<u32, Box<dyn Error>> {
    let (rules, orders) = read_input(path).unwrap();
    let total: u32 = orders
        .into_iter()
        .filter_map(|original_order| {
            let sorted_order = sort_by_rules(rules.clone(), original_order.clone()).unwrap();
            if original_order == sorted_order {
                None
            } else {
                Some(get_midpoint(&sorted_order))
            }
        })
        .sum();
    Ok(total)
}

fn main() {
    let path: &str = "data/day5.input";
    println!("Part 1: {}", part1(path).unwrap());
    println!("Part 2: {}", part2(path).unwrap());
}
