advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let re_a = regex::Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let re_b = regex::Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let re_prize = regex::Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let mut vec_a: Vec<(i64, i64)> = Vec::new();
    let mut vec_b: Vec<(i64, i64)> = Vec::new();
    let mut vec_prize: Vec<(i64, i64)> = Vec::new();

    for caps in re_a.captures_iter(input) {
        let x = caps[1].parse::<i64>().unwrap();
        let y = caps[2].parse::<i64>().unwrap();
        vec_a.push((x, y));
    }

    for caps in re_b.captures_iter(input) {
        let x = caps[1].parse::<i64>().unwrap();
        let y = caps[2].parse::<i64>().unwrap();
        vec_b.push((x, y));
    }

    for caps in re_prize.captures_iter(input) {
        let x = caps[1].parse::<i64>().unwrap();
        let y = caps[2].parse::<i64>().unwrap();
        vec_prize.push((x, y));
    }

    let mut tokens: u64 = 0;

    for i in 0..vec_a.len() {
        let (x1, y1) = vec_a[i];
        let (x2, y2) = vec_b[i];
        let (x3, y3) = vec_prize[i];
        if (x2 * y3 - y2 * x3) % (x2 * y1 - y2 * x1) != 0 {
            continue;
        }
        let a = (x2 * y3 - y2 * x3) / (x2 * y1 - y2 * x1);

        if (a * x1 - x3) % x2 != 0 {
            continue;
        }

        let b = (x3 - a * x1) / x2;
        tokens += (a * 3 + b) as u64;
    }

    Some(tokens as u32)
}

pub fn part_two(input: &str) -> Option<i128> {
    let re_a = regex::Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let re_b = regex::Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let re_prize = regex::Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let mut vec_a: Vec<(i128, i128)> = Vec::new();
    let mut vec_b: Vec<(i128, i128)> = Vec::new();
    let mut vec_prize: Vec<(i128, i128)> = Vec::new();

    for caps in re_a.captures_iter(input) {
        let x = caps[1].parse::<i128>().unwrap();
        let y = caps[2].parse::<i128>().unwrap();
        vec_a.push((x, y));
    }

    for caps in re_b.captures_iter(input) {
        let x = caps[1].parse::<i128>().unwrap();
        let y = caps[2].parse::<i128>().unwrap();
        vec_b.push((x, y));
    }

    for caps in re_prize.captures_iter(input) {
        let x = caps[1].parse::<i128>().unwrap() + 10000000000000;
        let y = caps[2].parse::<i128>().unwrap() + 10000000000000;
        vec_prize.push((x, y));
    }

    let mut tokens: i128 = 0;

    for i in 0..vec_a.len() {
        let (x1, y1) = vec_a[i];
        let (x2, y2) = vec_b[i];
        let (x3, y3) = vec_prize[i];
        if (x2 * y3 - y2 * x3) % (x2 * y1 - y2 * x1) != 0 {
            continue;
        }
        let a = (x2 * y3 - y2 * x3) / (x2 * y1 - y2 * x1);

        if (a * x1 - x3) % x2 != 0 {
            continue;
        }

        let b = (x3 - a * x1) / x2;
        tokens += a * 3 + b;
        //println!("Machine {} is possible", i);
    }

    Some(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
