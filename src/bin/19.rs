advent_of_code::solution!(19);
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
