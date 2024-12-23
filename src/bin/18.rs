advent_of_code::solution!(18);
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec_size: usize = 71;
    let mut kilo = 1024;
    if input.len() < 300 {
        vec_size = 7;
        kilo = 12;
    }
    let byte_coordinates: Vec<(u32, u32)> = input
        .split_whitespace()
        .map(|str| {
            let mut split = str.split(",");
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    let kilo_coords: Vec<(u32, u32)> = byte_coordinates[0..kilo].to_vec();
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for y in 0..vec_size {
        grid.push(Vec::new());
        for x in 0..vec_size {
            grid[y].push(false);
        }
    }
    for coord in kilo_coords {
        let (x, y) = coord;
        grid[y as usize][x as usize] = true;
    }
    let coord_map = bfs_search(&grid, (0, 0), vec_size as i32).expect("Did not find path");
    let mut length = 0;

    let mut grid_viz = grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|point| if *point { '#' } else { '.' })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut current_coord = (vec_size - 1, vec_size - 1);
    while current_coord != (0, 0) {
        grid_viz[current_coord.1][current_coord.0] = 'o';
        current_coord = *coord_map.get(&current_coord).unwrap();
        length += 1;
    }

    // println!("{}", char_matrix_to_string_matrix(&grid_viz));
    // println!("Length {}", length);
    return Some(length);
}

fn char_matrix_to_string_matrix(char_matrix: &Vec<Vec<char>>) -> String {
    return char_matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

fn bfs_search(
    grid: &Vec<Vec<bool>>,
    start_coord: (usize, usize),
    max: i32,
) -> Option<HashMap<(usize, usize), (usize, usize)>> {
    let mut searched: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut to_search: VecDeque<(usize, usize)> = VecDeque::new();
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    searched.insert(start_coord, (0, 0));
    to_search.push_back(start_coord);
    while !to_search.is_empty() {
        let coord = to_search.pop_front().unwrap();
        for dir in dirs {
            let x = coord.0 as i32 + dir.0;
            let y = coord.1 as i32 + dir.1;
            if !(searched.contains_key(&(x as usize, y as usize))
                || x < 0
                || y < 0
                || x >= max
                || y >= max)
                && !grid[y as usize][x as usize]
            {
                let new_coord = (x as usize, y as usize);
                searched.insert(new_coord, coord);
                if x == max - 1 && y == max - 1 {
                    return Some(searched);
                }
                to_search.push_back((x as usize, y as usize));
            }
        }
    }
    return None;
}

pub fn part_two(input: &str) -> Option<String> {
    let mut vec_size: usize = 71;
    let mut kilo = 1024;
    if input.len() < 300 {
        vec_size = 7;
        kilo = 12;
    }
    let byte_coordinates: Vec<(u32, u32)> = input
        .split_whitespace()
        .map(|str| {
            let mut split = str.split(",");
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    let mut kilo_coords: Vec<(u32, u32)> = byte_coordinates[0..kilo].to_vec();
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for y in 0..vec_size {
        grid.push(Vec::new());
        for x in 0..vec_size {
            grid[y].push(false);
        }
    }
    for coord in kilo_coords {
        let (x, y) = coord;
        grid[y as usize][x as usize] = true;
    }

    let mut i = 0;
    let mut j = 0;
    while bfs_search(&grid, (0, 0), vec_size as i32).is_some() {
        kilo += 1;
        (i, j) = byte_coordinates[kilo];
        grid[j as usize][i as usize] = true;
    }

    return Some(format!("{},{}", i, j));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
