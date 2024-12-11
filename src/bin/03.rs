use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let result = re.captures_iter(input).fold(0, |acc: u32, caps| {
        acc + (caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap())
    });
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|(do\(\))|(don't\(\))").unwrap();
    let result = re
        .captures_iter(input)
        .fold((0, true), |acc: (u32, bool), caps| match &caps[0] {
            "do()" => return (acc.0, true),
            "don't()" => return (acc.0, false),
            _ => {
                return (
                    acc.0
                        + (caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap())
                            * acc.1 as u32,
                    acc.1,
                )
            }
        });
    return Some(result.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, None);
    }
}
