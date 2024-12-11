advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let readings: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    let mut diff_readings: Vec<Vec<i32>> = readings
        .iter()
        .map(|line| {
            line.iter()
                .zip(line.iter().skip(1))
                .map(|(a, b)| a - b)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    diff_readings.retain(|line| {
        line.iter()
            .fold(true, |acc: bool, x| acc && x.abs() > 0 && x.abs() < 4)
    });
    diff_readings.retain(|line| {
        line.iter().sum::<i32>().abs() == line.iter().fold(0, |acc, x| acc + x.abs())
    });
    return Some(diff_readings.len() as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let readings: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    let correct: Vec<u32> = readings
        .iter()
        .map(|line| {
            for i in 0..line.len() {
                let diff_iter = line
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, x)| x);

                let diff = diff_iter
                    .clone()
                    .zip(diff_iter.skip(1))
                    .map(|(a, b)| a - b)
                    .collect::<Vec<i32>>();
                let step_cond = diff
                    .iter()
                    .fold(true, |acc: bool, x| acc && x.abs() > 0 && x.abs() < 4);
                let sign_cond =
                    diff.iter().sum::<i32>().abs() == diff.iter().fold(0, |acc, x| acc + x.abs());
                if step_cond && sign_cond {
                    return 1;
                }
            }
            return 0;
        })
        .collect();
    return Some(correct.iter().sum::<u32>() as u32);
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
