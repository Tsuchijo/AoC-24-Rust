use core::num;
use std::collections::HashSet;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut obstacle_matrix: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut guard_pos: (i32, i32) = (0, 0);
    for (i, row) in obstacle_matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                guard_pos = (i as i32, j as i32);
            }
        }
    }
    let height = obstacle_matrix.len() as i32;
    let width = obstacle_matrix[0].len() as i32;
    let mut guard_dir: (i32, i32) = (-1, 0);
    while guard_pos.0 < height && guard_pos.1 < width && guard_pos.0 >= 0 && guard_pos.1 >= 0 {
        obstacle_matrix[guard_pos.0 as usize][guard_pos.1 as usize] = 'X';
        let next_cell = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);
        if next_cell.0 < height && next_cell.1 < width && next_cell.0 >= 0 && next_cell.1 >= 0 {
            if obstacle_matrix[next_cell.0 as usize][next_cell.1 as usize] == '#' {
                guard_dir = match guard_dir {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => (0, 0),
                };
                guard_pos = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);
            } else {
                guard_pos = next_cell;
            }
        } else {
            break;
        }
    }
    //println!("{}", char_matrix_to_string_matrix(&obstacle_matrix));
    return Some(
        obstacle_matrix
            .iter()
            .flatten()
            .filter(|&c| *c == 'X')
            .count() as u32,
    );
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut obstacle_matrix: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut guard_pos: (i32, i32) = (0, 0);
    for (i, row) in obstacle_matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                guard_pos = (i as i32, j as i32);
            }
        }
    }
    let height = obstacle_matrix.len() as i32;
    let width = obstacle_matrix[0].len() as i32;
    let guard_dir: (i32, i32) = (-1, 0);
    let mut guard = (guard_pos, guard_dir);
    let mut num_loops = 0;
    let mut num_steps = 0;
    let mut checked_set: HashSet<(i32, i32)> = HashSet::new();
    while guard.0 .0 < height && guard.0 .1 < width && guard.0 .0 >= 0 && guard.0 .1 >= 0 {
        num_steps += 1;
        let mut test_guard: ((i32, i32), (i32, i32)) = guard.clone();
        match iterate_guard(guard, &mut obstacle_matrix) {
            Some(g) => guard = g,
            None => break,
        }
        let mut mod_obs_matrix = obstacle_matrix.clone();
        //let mut test_guard: ((i32, i32), (i32, i32)) = guard.clone();
        let next_cell: (i32, i32) = guard.0.clone();
        if next_cell.0 < height
            && next_cell.1 < width
            && next_cell.0 >= 0
            && next_cell.1 >= 0
            && !checked_set.contains(&next_cell)
        {
            mod_obs_matrix[next_cell.0 as usize][next_cell.1 as usize] = '#';
            let mut loop_set: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
            checked_set.insert(next_cell);
            while test_guard.0 .0 < height
                && test_guard.0 .1 < width
                && test_guard.0 .0 >= 0
                && test_guard.0 .1 >= 0
            {
                loop_set.insert(test_guard);
                // print current matrix with escape char to overwrite previous
                //print!("\r{}", char_matrix_to_string_matrix(&mod_obs_matrix));
                match iterate_guard(test_guard, &mut mod_obs_matrix) {
                    Some(g) => test_guard = g,
                    None => {
                        break;
                    }
                }
                if loop_set.contains(&test_guard) {
                    num_loops += 1;
                    break;
                }
            }
        }
        //println!("\n{}\n\n", char_matrix_to_string_matrix(&mod_obs_matrix));
    }
    //println!("{}", char_matrix_to_string_matrix(&obstacle_matrix));
    return Some(num_loops);
}

fn char_matrix_to_string_matrix(char_matrix: &Vec<Vec<char>>) -> String {
    return char_matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

fn iterate_guard(
    guard: ((i32, i32), (i32, i32)),
    obstacle_matrix: &mut Vec<Vec<char>>,
) -> Option<((i32, i32), (i32, i32))> {
    let mut guard_pos = guard.0;
    let mut guard_dir = guard.1;
    let height = obstacle_matrix.len() as i32;
    let width = obstacle_matrix[0].len() as i32;
    let next_cell: (i32, i32) = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);
    obstacle_matrix[guard_pos.0 as usize][guard_pos.1 as usize] = 'X';
    if next_cell.0 < height && next_cell.1 < width && next_cell.0 >= 0 && next_cell.1 >= 0 {
        let mut next_char = obstacle_matrix[next_cell.0 as usize][next_cell.1 as usize];
        if next_char == '#' {
            while next_char == '#' {
                guard_dir = match guard_dir {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => (0, 0),
                };
                let next_cell: (i32, i32) = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);
                if next_cell.0 < height
                    && next_cell.1 < width
                    && next_cell.0 >= 0
                    && next_cell.1 >= 0
                {
                    next_char = obstacle_matrix[next_cell.0 as usize][next_cell.1 as usize];
                } else {
                    break;
                }
            }
            guard_pos = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);
        } else {
            guard_pos = next_cell;
        }
    } else {
        return None;
    }
    return Some((guard_pos, guard_dir));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        //let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        //let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
