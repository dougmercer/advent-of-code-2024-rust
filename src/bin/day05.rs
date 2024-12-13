use itertools::Itertools;
use std::{error::Error, fs};
use topological_sort::TopologicalSort;

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    // Split content into two parts on double newline
    let (rules_str, pages_str) = input.split_once("\n\n").unwrap_or((&input, ""));

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

    (rules, pages)
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

fn part1(input: &str) -> u32 {
    let (rules, orders) = parse_input(input);
    orders
        .into_iter()
        .filter_map(|original_order| {
            let sorted_order = sort_by_rules(rules.clone(), original_order.clone()).unwrap();
            if original_order == sorted_order {
                Some(get_midpoint(&sorted_order))
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let (rules, orders) = parse_input(input);
    orders
        .into_iter()
        .filter_map(|original_order| {
            let sorted_order = sort_by_rules(rules.clone(), original_order.clone()).unwrap();
            if original_order == sorted_order {
                None
            } else {
                Some(get_midpoint(&sorted_order))
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day5.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
    Ok(())
}

#[test]
fn test_part1() {
    let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    assert_eq!(part1(&input), 143);
}

#[test]
fn test_part2() {
    let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    assert_eq!(part2(&input), 123);
}
