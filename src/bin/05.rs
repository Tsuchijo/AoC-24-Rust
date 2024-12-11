use regex::Regex;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let re_rules = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap();
    let re_sequences = Regex::new(r"((([0-9]+),)+[0-9]+)").unwrap();
    let rules: Vec<(u32, u32)> = re_rules
        .captures_iter(input)
        .filter_map(|caps| {
            if caps[1].parse::<u32>().is_err() || caps[2].parse::<u32>().is_err() {
                return None;
            }
            Some((caps[1].parse().unwrap(), caps[2].parse().unwrap()))
        })
        .collect::<Vec<(u32, u32)>>();
    let sequences: Vec<Vec<u32>> = re_sequences
        .captures_iter(input)
        .map(|caps| {
            caps[1]
                .split(",")
                .filter_map(|x| {
                    if x.parse::<u32>().is_err() {
                        return None;
                    }
                    Some(x.parse().unwrap())
                })
                .collect()
        })
        .collect();
    let is_good: Vec<bool> = sequences
        .iter()
        .map(|sequence| {
            for rule in &rules {
                let rules_indices: Vec<usize> = sequence
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if x == &rule.0 {
                            return Some(i);
                        }
                        return None;
                    })
                    .collect();
                for rule_index in rules_indices {
                    for i in 0..rule_index {
                        if sequence[i] == rule.1 {
                            return false;
                        }
                    }
                }
            }
            return true;
        })
        .collect();
    let middle_number: Vec<u32> = sequences
        .iter()
        .zip(is_good.iter())
        .filter_map(|(sequence, is_good)| {
            if !is_good {
                return None;
            }
            return Some(sequence[sequence.len() / 2]);
        })
        .collect();

    return Some(middle_number.iter().sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_rules = Regex::new(r"([0-9]+)\|([0-9]+)").unwrap();
    let re_sequences = Regex::new(r"((([0-9]+),)+[0-9]+)").unwrap();
    let rules: Vec<(u32, u32)> = re_rules
        .captures_iter(input)
        .filter_map(|caps| {
            if caps[1].parse::<u32>().is_err() || caps[2].parse::<u32>().is_err() {
                return None;
            }
            Some((caps[1].parse().unwrap(), caps[2].parse().unwrap()))
        })
        .collect::<Vec<(u32, u32)>>();

    let mut sequences: Vec<Vec<u32>> = re_sequences
        .captures_iter(input)
        .map(|caps| {
            caps[1]
                .split(",")
                .filter_map(|x| {
                    if x.parse::<u32>().is_err() {
                        return None;
                    }
                    Some(x.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let mut is_good: Vec<(bool, Vec<(usize, usize)>)> = find_good(&rules, &sequences);

    sequences = sequences
        .iter_mut()
        .zip(is_good.iter())
        .filter_map(|(sequence, is_good)| {
            if !is_good.0 {
                let (i, j) = is_good.1[0];
                let temp = sequence[i];
                if j < sequence.len() {
                    sequence.insert(j, temp);
                } else {
                    sequence.push(temp);
                }
                sequence.remove(i);
                return Some(sequence.clone());
            }
            return None;
        })
        .collect();
    is_good = find_good(&rules, &sequences);
    while is_good.iter().any(|x| !x.0) {
        sequences = sequences
            .iter_mut()
            .zip(is_good.iter())
            .map(|(sequence, is_good)| {
                if !is_good.0 {
                    let (i, j) = is_good.1[0];
                    sequence.swap(i, j);
                }
                return sequence.clone();
            })
            .collect();
        is_good = find_good(&rules, &sequences);
    }

    let middle_number: Vec<u32> = sequences
        .iter()
        .filter_map(|sequence| {
            return Some(sequence[sequence.len() / 2]);
        })
        .collect();

    return Some(middle_number.iter().sum());
}

fn find_good(
    rules: &Vec<(u32, u32)>,
    sequences: &Vec<Vec<u32>>,
) -> Vec<(bool, Vec<(usize, usize)>)> {
    return sequences
        .iter()
        .map(|sequence| {
            let mut swap_indices: Vec<(usize, usize)> = Vec::new();
            for rule in rules {
                let rules_indices: Vec<usize> = sequence
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if x == &rule.0 {
                            return Some(i);
                        }
                        return None;
                    })
                    .collect();

                for rule_index in rules_indices {
                    for i in 0..rule_index {
                        if sequence[i] == rule.1 {
                            swap_indices.push((i, rule_index));
                        }
                    }
                }
            }
            return (swap_indices.len() == 0, swap_indices);
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(122));
        // print!("Result: {:?}", result);
    }
}
