use core::num;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let maze: Vec<Vec<char>> = input.lines().map(|s: &str| s.chars().collect()).collect();
    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);
    for (j, row) in maze.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c == 'S' {
                start_pos = (i, j);
            }
            if *c == 'E' {
                end_pos = (i, j);
            }
        }
    }
    let defaut_path_time = bfs_search(&maze, start_pos, end_pos, 0xFFFFFFFF).unwrap();
    println!("Default path time: {}", defaut_path_time);
    let num_cheats = double_ended_search(&maze, start_pos, end_pos, defaut_path_time - 100, 2);
    return Some(num_cheats);
}

fn get_adjacent(coord: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut adjacents: Vec<(usize, usize)> = Vec::new();
    let (x, y) = coord;
    if x > 0 {
        adjacents.push((x - 1, y));
    }
    if y > 0 {
        adjacents.push((x, y - 1));
    }
    if x < width - 1 {
        adjacents.push((x + 1, y));
    }
    if y < height - 1 {
        adjacents.push((x, y + 1));
    }
    return adjacents;
}

fn bfs_search(
    grid: &Vec<Vec<char>>,
    start_coord: (usize, usize),
    end_coord: (usize, usize),
    max_time: u32,
) -> Option<u32> {
    let height = grid.len();
    let width = grid[0].len();
    let mut to_search: VecDeque<((usize, usize), u32)> = VecDeque::new();
    let mut searched: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    to_search.push_back((start_coord, 0));
    while !to_search.is_empty() {
        let (coord, time) = to_search.pop_front().unwrap();
        if time >= max_time {
            return None;
        }
        for adj in get_adjacent(coord, width, height) {
            if grid[adj.1][adj.0] != '#' && !searched.contains_key(&adj) {
                searched.insert(adj, coord);
                if adj == end_coord {
                    let mut path = 0;
                    let mut current = adj;
                    while current != start_coord {
                        path += 1;
                        current = searched[&current];
                    }
                    return Some(path);
                }
                to_search.push_back((adj, time + 1));
            }
        }
    }
    return None;
}

fn get_within_distance(
    coord: (usize, usize),
    width: usize,
    height: usize,
    distance: i32,
) -> Vec<((u32, u32), u32)> {
    let mut adjacents: Vec<((u32, u32), u32)> = Vec::new();
    // Check each possible dx, dy combination within Manhattan distance
    let (start_x, start_y) = coord;
    for dx in -distance..=distance {
        let remaining_distance = distance - dx.abs();
        for dy in -remaining_distance..=remaining_distance {
            let new_x = start_x as i32 + dx;
            let new_y = start_y as i32 + dy;

            // Ensure point is within grid bounds
            if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32 {
                adjacents.push(((new_x as u32, new_y as u32), (dx.abs() + dy.abs()) as u32));
            }
        }
    }

    return adjacents;
}

fn get_search_map(
    grid: &Vec<Vec<char>>,
    start_coord: (usize, usize),
    end_coord: (usize, usize),
) -> HashMap<(u32, u32), u32> {
    let height = grid.len();
    let width = grid[0].len();
    let mut to_search: VecDeque<((usize, usize), u32)> = VecDeque::new();
    let mut searched_distance: HashMap<(u32, u32), u32> = HashMap::new();
    to_search.push_back((end_coord, 0));
    searched_distance.insert((end_coord.0 as u32, end_coord.1 as u32), 0);
    while !to_search.is_empty() {
        let (coord, time) = to_search.pop_front().unwrap();
        for adj in get_adjacent(coord, width, height) {
            if grid[adj.1][adj.0] != '#'
                && !searched_distance.contains_key(&(adj.0 as u32, adj.1 as u32))
            {
                searched_distance.insert((adj.0 as u32, adj.1 as u32), time + 1);
                to_search.push_back((adj, time + 1));
            }
        }
    }
    return searched_distance;
}

fn double_ended_search(
    grid: &Vec<Vec<char>>,
    start_coord: (usize, usize),
    end_coord: (usize, usize),
    max_time: u32,
    cheat_dist: i32,
) -> u32 {
    let start_to_end = get_search_map(&grid, start_coord, end_coord);
    let end_to_start = get_search_map(&grid, end_coord, start_coord);
    let mut num_cheats = 0;
    for (coord, time) in start_to_end {
        let (x, y) = coord;
        for cheat in get_within_distance(
            (x as usize, y as usize),
            grid[0].len(),
            grid.len(),
            cheat_dist,
        ) {
            if let Some(end_time) = end_to_start.get(&cheat.0) {
                if time + cheat.1 + end_time <= max_time {
                    num_cheats += 1;
                }
            }
        }
    }
    return num_cheats;
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze: Vec<Vec<char>> = input.lines().map(|s: &str| s.chars().collect()).collect();
    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);
    for (j, row) in maze.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c == 'S' {
                start_pos = (i, j);
            }
            if *c == 'E' {
                end_pos = (i, j);
            }
        }
    }
    let defaut_path_time = bfs_search(&maze, start_pos, end_pos, 0xFFFFFFFF).unwrap();
    let num_cheats = double_ended_search(&maze, start_pos, end_pos, defaut_path_time - 100, 20);
    return Some(num_cheats);
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
