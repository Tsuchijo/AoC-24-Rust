use std::{collections::HashSet, hash::Hash};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let char_matrix = input
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = char_matrix[0].len();
    let height = char_matrix.len();
    let mut searched: HashSet<(u32, u32)> = HashSet::new();
    let mut price = 0;
    for i in 0..height as u32 {
        for j in 0..width as u32 {
            let char = char_matrix[i as usize][j as usize];
            if searched.contains(&(i, j)) {
                continue;
            }
            let mut patch: HashSet<(u32, u32)> = HashSet::new();
            let (perimeter, area) =
                find_all_patch(i as i32, j as i32, char, &char_matrix, &mut patch);
            searched.extend(patch);
            price += perimeter * area;
        }
    }
    return Some(price);
}

fn find_all_patch(
    i: i32,
    j: i32,
    char_id: char,
    char_matrix: &Vec<Vec<char>>,
    patch: &mut HashSet<(u32, u32)>,
) -> (u32, u32) {
    let width = char_matrix[0].len() as i32;
    let height = char_matrix.len() as i32;
    if i >= height || j >= width || i < 0 || j < 0 || char_matrix[i as usize][j as usize] != char_id
    {
        return (1, 0);
    } else {
        patch.insert((i as u32, j as u32));
    }
    let mut perimeter = 0;
    let mut area: u32 = 1;
    for (dx, dy) in [(0, 1), (0, -1_i32), (1, 0), (-1_i32, 0)] {
        if patch.contains(&((i + dx) as u32, (j + dy) as u32)) {
            continue;
        }
        let (adj, run_area) = find_all_patch(i + dx, j + dy, char_id, char_matrix, patch);
        perimeter += adj;
        area += run_area;
    }
    return (perimeter, area);
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_matrix = input
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let width = char_matrix[0].len();
    let height = char_matrix.len();
    let mut patches: Vec<HashSet<(u32, u32)>> = Vec::new();
    let mut areas: Vec<u32> = Vec::new();
    let mut searched: HashSet<(u32, u32)> = HashSet::new();
    for i in 0..height as u32 {
        for j in 0..width as u32 {
            let char = char_matrix[i as usize][j as usize];
            if searched.contains(&(i, j)) {
                continue;
            }
            let mut patch: HashSet<(u32, u32)> = HashSet::new();
            let (perimeter, area) =
                find_all_patch(i as i32, j as i32, char, &char_matrix, &mut patch);
            patches.push(patch.clone());
            searched.extend(patch);
            areas.push(area);
        }
    }

    let sides = patches
        .iter()
        .map(|x| find_number_sides(x.clone()))
        .collect::<Vec<u32>>();

    for (sides, area) in sides.iter().zip(areas.iter()) {
        println!("{:?} {:?}", sides, area);
    }

    return Some(
        areas
            .iter()
            .zip(sides.iter())
            .map(|(x, y)| x * y)
            .sum::<u32>(),
    );
}

fn find_number_sides(patch: HashSet<(u32, u32)>) -> u32 {
    if patch.len() == 1 {
        return 4;
    }
    let starting_point: (u32, u32) = *patch.iter().reduce(|acc, coords| {
        if coords.0 > acc.0 {
            return  coords;
        }
        if coords.0 == acc.0 && coords.1 > acc.1 {
            return coords;
        }
        return acc;
    }).unwrap();
    let mut current_pos = starting_point;
    let mut num_sides = 0;
    let mut dir = (0, -1);
    while num_sides == 0 || current_pos != starting_point {
        //println!("current pos: {:?}", current_pos);
        let (_, right) = get_left_right(dir);
        let right_pos = ((current_pos.0 as i32 + right.0) as u32, (current_pos.1 as i32 + right.1) as u32);
        let forward_pos = ((current_pos.0 as i32 + dir.0) as u32, (current_pos.1 as i32 + dir.1) as u32);
        if patch.contains(&right_pos) {
            num_sides += 1;
            current_pos = right_pos;
            dir = right;
        }
        else if patch.contains(&forward_pos) {
            current_pos = forward_pos;
        }
        else {
            for rotation in get_left_rotation(dir) {
                num_sides += 1;
                let rotate_pos = ((current_pos.0 as i32 + rotation.0) as u32, (current_pos.1 as i32 + rotation.1) as u32);
                if patch.contains(&rotate_pos) {
                    current_pos = rotate_pos;
                    dir = rotation;
                    break
                }
            }
        }
        
    }
    return num_sides;
}

fn get_left_right(direction: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    return match direction {
        (1,0) => ((0, -1), (0 ,1)),
        (-1,0) => ((0, 1), (0 ,-1)),
        (0,1) => ((1, 0), (-1 ,0)),
        (0,-1) => ((-1, 0), (1 ,0)),
        _ => ((0, -1), (0 ,1))
    };
}

fn get_left_rotation(direction: (i32, i32)) -> Vec<(i32, i32)> {
    return match direction {
        (1,0) => vec![(0, -1), (-1 , 0)],
        (-1,0) => vec![(0, 1), (1 ,0)],
        (0,1) => vec![(1, 0), (0 ,-1)],
        (0,-1) => vec![(-1, 0), (0 ,1)],
        _ => vec![(-1, 0), (0 ,1)]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
