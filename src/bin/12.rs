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
    let mut sides: HashSet<Vec<(i32, i32)>> = HashSet::new();
    let patch: HashSet<(i32, i32)> = remove_all_interior(patch);
    for (i, j) in patch.iter() {
        if !patch.contains(&(*i + 1, *j)) {
            sides.insert(find_adjacent_x(patch.clone(), (*i, *j)));
        }
        if !patch.contains(&(*i, *j + 1)) {
            sides.insert(find_adjacent_y(patch.clone(), (*i, *j)));
        }
    }
    // remove all sides of length 1
    sides.retain(|x| x.len() > 1);
    println!("{:?}", sides);
    return sides.len() as u32;
}

fn find_adjacent_x(patch: HashSet<(i32, i32)>, start: (i32, i32)) -> Vec<(i32, i32)> {
    let mut side: Vec<(i32, i32)> = Vec::new();
    side.push(start);
    let mut counter = 1;
    let mut is_more = true;
    while is_more {
        is_more = false;
        if patch.contains(&(start.0 + counter, start.1)) {
            side.push((start.0 + counter, start.1));
            is_more = true;
        }
        if patch.contains(&(start.0 - counter, start.1)) {
            side.push((start.0 - counter, start.1));
            is_more = true;
        }
        counter += 1;
    }
    side.sort();
    return side;
}

fn find_adjacent_y(patch: HashSet<(i32, i32)>, start: (i32, i32)) -> Vec<(i32, i32)> {
    let mut side: Vec<(i32, i32)> = Vec::new();
    side.push(start);
    let mut counter = 1;
    let mut is_more = true;
    while is_more {
        is_more = false;
        if patch.contains(&(start.0, start.1 + counter)) {
            side.push((start.0, start.1 + counter));
            is_more = true;
        }
        if patch.contains(&(start.0, start.1 - counter)) {
            side.push((start.0, start.1 - counter));
            is_more = true;
        }
        counter += 1;
    }
    side.sort();
    return side;
}

fn remove_all_interior(patch: HashSet<(u32, u32)>) -> HashSet<(i32, i32)> {
    let mut new_patch: HashSet<(i32, i32)> = HashSet::new();
    for (i, j) in patch.iter() {
        new_patch.insert((*i as i32, *j as i32));
        new_patch.insert((*i as i32 + 1, *j as i32));
        new_patch.insert((*i as i32, *j as i32 + 1));
        new_patch.insert((*i as i32 - 1, *j as i32));
        new_patch.insert((*i as i32, *j as i32 - 1));
        new_patch.insert((*i as i32 + 1, *j as i32 + 1));
        new_patch.insert((*i as i32 - 1, *j as i32 - 1));
        new_patch.insert((*i as i32 + 1, *j as i32 - 1));
        new_patch.insert((*i as i32 - 1, *j as i32 + 1));
    }

    for (i, j) in patch.iter() {
        if new_patch.contains(&(*i as i32 + 1, *j as i32))
            && new_patch.contains(&(*i as i32 - 1, *j as i32))
            && new_patch.contains(&(*i as i32, *j as i32 + 1))
            && new_patch.contains(&(*i as i32, *j as i32 - 1))
        {
            new_patch.remove(&(*i as i32, *j as i32));
        }
    }
    return new_patch;
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
