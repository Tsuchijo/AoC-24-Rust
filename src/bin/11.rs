advent_of_code::solution!(11);
use std::collections::HashMap;
use std::{
    hash::Hash,
    sync::{Arc, Mutex},
    thread,
};

pub fn part_one(input: &str) -> Option<u64> {
    let mut input = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    for _ in 0..25 {
        input = input.iter().flat_map(|x| evolve_stone(*x)).collect();
    }

    return Some(input.len() as u64);
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    let stones_25_step = evolve_n_steps(input, 25, 8);
    let mut value_dict: HashMap<u64, u64> = HashMap::new();
    let mut num_stones: u64 = 0;
    for (i, stones) in stones_25_step.chunks(32).enumerate() {
        print!(
            "progress: {} \r",
            (i as f64) * 32.0 / stones_25_step.len() as f64
        );
        let stones_50 = evolve_n_steps(stones.to_vec(), 25, 8);
        for stones_2 in stones_50 {
            if value_dict.contains_key(&stones_2) {
                num_stones += value_dict.get(&stones_2).unwrap();
            } else {
                let num_stones_75 = evolve_n_steps(vec![stones_2], 25, 1).len() as u64;
                num_stones += num_stones_75;
                value_dict.insert(stones_2, num_stones_75);
            }
        }
    }

    return Some(num_stones);
}

fn evolve_n_steps(stones: Vec<u64>, n: u64, threads: u32) -> Vec<u64> {
    let mut handles = Vec::new();
    let mut final_stones = Vec::new();
    let partition_size = stones.len() / threads as usize;
    for chunk in stones.chunks(partition_size) {
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || {
            let mut new_input = chunk.to_vec();
            for i in 0..n {
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
