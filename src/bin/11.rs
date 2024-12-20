advent_of_code::solution!(11);
use std::collections::HashMap;
use std::thread;

pub fn part_one(input: &str) -> Option<u64> {
    let input = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    let size = evolve_n_steps(input, 25, 8).len();

    return Some(size as u64);
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    let mut score_dict = HashMap::new();
    let result = input.iter().map(|x| score(*x, 0, &mut score_dict)).sum();

    return Some(result);
}

fn evolve_n_steps(stones: Vec<u64>, n: u64, threads: u32) -> Vec<u64> {
    let mut handles = Vec::new();
    let mut final_stones = Vec::new();
    let partition_size = stones.len() / threads as usize;
    for chunk in stones.chunks(partition_size) {
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || {
            let mut new_input = chunk.to_vec();
            for _ in 0..n {
                new_input = new_input.iter().flat_map(|x| evolve_stone(*x)).collect();
            }
            new_input
        }));
    }

    for handle in handles {
        final_stones.push(handle.join().unwrap());
    }
    return final_stones.into_iter().flatten().collect();
}

fn score(stone: u64, blinks: u32, score_dict: &mut HashMap<(u64, u32), u64>) -> u64 {
    if score_dict.contains_key(&(stone, blinks)) {
        return *score_dict.get(&(stone, blinks)).unwrap();
    } else if blinks == 75 {
        let size = 1;
        score_dict.insert((stone, blinks), size);
        return size;
    } else if stone == 0 {
        let size = score(1, blinks + 1, score_dict);
        score_dict.insert((stone, blinks), size);
        return size;
    } else if stone.to_string().len() % 2 == 0 {
        let stone_string: String = stone.to_string();
        let len = stone_string.len();
        let left: u64 = stone_string[..len / 2].parse().unwrap();
        let right: u64 = stone_string[len / 2..].parse().unwrap();
        let size = score(left, blinks + 1, score_dict) + score(right, blinks + 1, score_dict);
        score_dict.insert((stone, blinks), size);
        return size;
    } else {
        let size = score(stone * 2024, blinks + 1, score_dict);
        score_dict.insert((stone, blinks), size);
        return size;
    }
}

fn evolve_stone(stone: u64) -> Vec<u64> {
    let stone_string: String = stone.to_string();
    let len = stone_string.len();
    if stone == 0 {
        return vec![1];
    }
    if len % 2 == 0 {
        let left: u64 = stone_string[..len / 2].parse().unwrap();
        let right: u64 = stone_string[len / 2..].parse().unwrap();
        return vec![left, right];
    }
    return vec![stone * 2024];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        //     assert_eq!(true, true);
    }
}
