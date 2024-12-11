advent_of_code::solution!(7);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"([0-9]+): (([0-9]+(\ |\n))+)").unwrap();
    let results: Vec<u64> = re
        .captures_iter(input)
        .map(|caps| {
            let cap1 = caps[1].to_string();
            cap1.parse::<u64>()
                .unwrap_or_else(|_| panic!("Could not parse result: {}", cap1))
        })
        .collect();

    let equations: Vec<Vec<u64>> = re
        .captures_iter(input)
        .map(|caps| {
            caps[2]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let final_sum = results
        .iter()
        .zip(equations.iter())
        .filter_map(|(result, equation)| {
            //println!("{:?}", result);
            for i in 0..2_u64.pow((equation.len() - 1) as u32) {
                let mut permutation = format!("{i:b}");
                if permutation.len() < equation.len() - 1 {
                    permutation = format!("{:0>1$}", permutation, equation.len() - 1);
                }

                let test_result = equation.iter().skip(1).zip(permutation.chars()).fold(
                    equation[0],
                    |acc, (x, c)| {
                        if c == '1' {
                            return acc * *x;
                        }
                        return acc + *x;
                    },
                );
                if test_result == *result {
                    return Some(*result);
                }
            }
            None
        })
        .sum();
    return Some(final_sum);
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"([0-9]+): (([0-9]+(\ |\n))+)").unwrap();
    let results: Vec<u64> = re
        .captures_iter(input)
        .map(|caps| {
            let cap1 = caps[1].to_string();
            cap1.parse::<u64>()
                .unwrap_or_else(|_| panic!("Could not parse result: {}", cap1))
        })
        .collect();

    let equations: Vec<Vec<u64>> = re
        .captures_iter(input)
        .map(|caps| {
            caps[2]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let final_sum = results
        .iter()
        .zip(equations.iter())
        .filter_map(|(result, equation)| {
            for i in 0..3_u64.pow((equation.len() - 1) as u32) {
                let permutation = create_trinary_string(i, equation.len() as u64 - 1);
                let test_result = equation.iter().skip(1).zip(permutation.chars()).fold(
                    equation[0],
                    |acc, (x, c)| {
                        if c == '1' {
                            return acc * *x;
                        } else if c == '2' {
                            return acc * 10_u64.pow(x.to_string().len() as u32) + *x;
                        }
                        return acc + *x;
                    },
                );
                if test_result == *result {
                    return Some(*result);
                }
            }
            None
        })
        .sum();
    return Some(final_sum);
}

fn create_trinary_string(n: u64, len: u64) -> String {
    let mut inter_n = n;
    let mut result = 0;
    let mut iter = 0;
    while inter_n > 0 {
        let modulo_3 = inter_n % 3;
        result += modulo_3 * 10_u64.pow(iter);
        inter_n /= 3;
        iter += 1;
    }
    let string_result = result.to_string();
    if string_result.len() < len as usize {
        return format!("{:0>1$}", string_result, len as usize);
    }
    return string_result;
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
        // assert_eq!(result, None);
    }
}
