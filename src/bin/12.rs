use std::{collections::HashSet, hash::Hash};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let char_matrix = input
        .split_whitespace()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = char_matrix[0].len();
    let height = char_matrix.len();
    let mut searched: HashSet<(i32, i32)> = HashSet::new();
    let mut price = 0;
    for i in 0..height as i32 {
        for j in 0..width as i32 {
            let char = char_matrix[i as usize][j as usize];
            if searched.contains(&(i, j)) {
                continue;
            }
            let mut patch: HashSet<(i32, i32)> = HashSet::new();
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
    patch: &mut HashSet<(i32, i32)>,
) -> (u32, u32) {
    let width = char_matrix[0].len() as i32;
    let height = char_matrix.len() as i32;
    if i >= height || j >= width || i < 0 || j < 0 || char_matrix[i as usize][j as usize] != char_id
    {
        return (1, 0);
    } else {
        patch.insert((i, j));
    }
    let mut perimeter = 0;
    let mut area: u32 = 1;
    for (dx, dy) in [(0, 1), (0, -1_i32), (1, 0), (-1_i32, 0)] {
        if patch.contains(&((i + dx), (j + dy))) {
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
    let mut patches: Vec<HashSet<(i32, i32)>> = Vec::new();
    let mut areas: Vec<u32> = Vec::new();
    let mut searched: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..height as i32 {
        for j in 0..width as i32 {
            let char = char_matrix[i as usize][j as usize];
            if searched.contains(&(i, j)) {
                continue;
            }
            let mut patch: HashSet<(i32, i32)> = HashSet::new();
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

fn find_number_sides(patch: HashSet<(i32, i32)>) -> u32 {
    let mut num_sides = 0;
    for (i, j) in patch.iter() {
        for corner in 0..4 {
            match check_corner(*i, *j, &patch, corner) {
                (true, false, false, false) => {
                    num_sides += 1;
                }
                (true, true, true, false) => {
                    num_sides += 1;
                }
                (true, false, false, true) => {
                    num_sides += 1;
                }
                _ => {}
            }
        }
    }
    return num_sides;
}

fn check_corner(
    i: i32,
    j: i32,
    patch: &HashSet<(i32, i32)>,
    corner: u32,
) -> (bool, bool, bool, bool) {
    if corner == 0 {
        return (
            patch.contains(&(i, j)),
            patch.contains(&(i + 1, j)),
            patch.contains(&(i, j + 1)),
            patch.contains(&(i + 1, j + 1)),
        );
    } else if corner == 1 {
        return (
            patch.contains(&(i, j)),
            patch.contains(&(i - 1, j)),
            patch.contains(&(i, j + 1)),
            patch.contains(&(i - 1, j + 1)),
        );
    } else if corner == 2 {
        return (
            patch.contains(&(i, j)),
            patch.contains(&(i + 1, j)),
            patch.contains(&(i, j - 1)),
            patch.contains(&(i + 1, j - 1)),
        );
    } else {
        return (
            patch.contains(&(i, j)),
            patch.contains(&(i - 1, j)),
            patch.contains(&(i, j - 1)),
            patch.contains(&(i - 1, j - 1)),
        );
    }
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
        assert_eq!(result, Some(368));
    }
}
