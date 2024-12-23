advent_of_code::solution!(19);
use std::{collections::HashMap, rc::Rc};

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let towels: Vec<String> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|str| str.replace(" ", ""))
        .collect();
    let patterns: Vec<&str> = input.lines().skip(2).collect();
    let possible_patterns: Vec<bool> = patterns
        .iter()
        .map(|pattern| check_pattern(&towels, pattern))
        .collect();
    return Some(possible_patterns.iter().fold(0, |acc, x| acc + *x as u32));
}

fn check_pattern(towels: &Vec<String>, pattern: &str) -> bool {
    let mut is_possible = false;
    for towel in towels {
        if towel.len() > pattern.len() {
            continue;
        }
        //println!("Candidate: {:?}, Towel: {:?}", &&pattern, towel);
        if &&pattern[0..towel.len()] == towel {
            if towel.len() == pattern.len() {
                return true;
            }
            is_possible = check_pattern(towels, &pattern[towel.len()..pattern.len()]);
            if is_possible {
                return true;
            }
        }
    }
    return is_possible;
}

fn get_number_of_matches<'a>(
    towels: &'a Vec<String>,
    pattern: String,
    seen_patterns: &'a mut HashMap<String, u64>,
) -> (u64, &'a mut HashMap<String, u64>) {
    let mut num_matches: u64 = 0;
    if seen_patterns.contains_key(&pattern) {
        return (*seen_patterns.get(&pattern).unwrap(), seen_patterns);
    }
    for towel in towels {
        if towel.len() > pattern.len() {
            continue;
        }
        if &&pattern[0..towel.len()] == towel {
            if towel.len() == pattern.len() {
                num_matches += 1;
            } else {
                let (x, seen_patterns) = get_number_of_matches(
                    towels,
                    pattern[towel.len()..pattern.len()].to_string(),
                    seen_patterns,
                );
                num_matches += x;
            }
        }
    }
    //println!("Pattern: {:?}, Matches: {:?}", pattern, num_matches);
    seen_patterns.insert(pattern, num_matches);
    return (num_matches, seen_patterns);
}

pub fn part_two(input: &str) -> Option<u64> {
    let towels: Vec<String> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|str| str.replace(" ", ""))
        .collect();
    let patterns: Vec<&str> = input.lines().skip(2).collect();
    let mut seen_patterns: HashMap<String, u64> = HashMap::new();
    let possible_patterns: Vec<u64> = patterns
        .iter()
        .map(|pattern| {
            let (x, new_seen_patterns) =
                get_number_of_matches(&towels, pattern.to_string(), &mut seen_patterns);
            seen_patterns = new_seen_patterns.clone();
            return x;
        })
        .collect();
    return Some(possible_patterns.iter().sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
