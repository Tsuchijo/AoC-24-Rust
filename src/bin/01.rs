advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut num_1: Vec<i32> = numbers.iter().step_by(2).copied().collect();
    let mut num_2: Vec<i32> = numbers.iter().skip(1).step_by(2).copied().collect();
    num_1.sort();
    num_2.sort();
    let diff: Vec<u32> = num_1
        .iter()
        .zip(num_2.iter())
        .map(|(a, b)| (a - b).abs() as u32)
        .collect();
    return Some(diff.iter().sum());
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let left_list: Vec<u32> = numbers.iter().step_by(2).copied().collect();
    let right_list: Vec<u32> = numbers.iter().skip(1).step_by(2).copied().collect();
    return Some(left_list.iter().fold(0, |acc, x| {
        acc + (right_list
            .iter()
            .filter(|y| *y == x)
            .collect::<Vec<&u32>>()
            .len() as u32
            * x)
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, None);
    }
}
