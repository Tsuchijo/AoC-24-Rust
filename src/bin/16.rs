use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let mut maze: Vec<Vec<char>> = input.lines().map(|s: &str| s.chars().collect()).collect();
    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);
    for (i, row) in maze.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                start_pos = (i, j);
            }
            if *c == 'E' {
                end_pos = (i, j);
            }
        }
    }
    println!("Start: {:?},  Finish: {:?}", start_pos, end_pos);
    let finished_paths = bfs_search(&maze, start_pos, end_pos);
    let low_score = finished_paths.iter().fold(0xFFFFFFFF, |acc, path| acc.min(path.score));
    //println!("{:?}", finished_paths);
    return Some(low_score);
}

#[derive(Clone)]
#[derive(Eq, Hash, PartialEq, Debug)]
struct PathTree {
    score: u32,
    coordinate: (usize, usize),
    direction: (i32, i32),
    parent: Option<Box<PathTree>>
}

fn bfs_search(grid: &Vec<Vec<char>>, start_coord: (usize, usize), end_coord: (usize, usize)) -> Vec<PathTree> {
    let mut to_search: VecDeque<PathTree> = VecDeque::new();
    let mut end_paths: Vec<PathTree> = Vec::new();
    let mut searched: HashSet<((usize, usize), (i32, i32))> = HashSet::new();
    to_search.push_back(PathTree{
        score: 0,
        coordinate: start_coord,
        direction: (1,0),
        parent: None
    });
    while !to_search.is_empty() {
        let coord_tree = to_search.pop_front().unwrap();
        for dir in get_rotations(coord_tree.direction) {
            let x = coord_tree.coordinate.0 as i32 + dir.0;
            let y = coord_tree.coordinate.1 as i32 + dir.1;
            if grid[y as usize][x as usize ] != '#' {
                let new_coord = (x as usize, y as usize);

                let mut new_path = PathTree{
                    score: &coord_tree.score + 1,
                    coordinate: (x as usize, y as usize),
                    direction: dir,
                    parent: Some(Box::new(coord_tree.clone()))
                };

                if searched.contains(&(new_path.coordinate, new_path.direction)) {
                    continue;
                }

                if dir != coord_tree.direction {
                    new_path.score += 1000;
                }
                if new_coord == end_coord {
                    end_paths.push(new_path);
                }
                else{
                    to_search.push_back(new_path);
                }
            }
        }
        searched.insert((coord_tree.coordinate, coord_tree.direction));
    }
    return end_paths;
}

fn get_rotations(direction: (i32, i32)) -> Vec<(i32, i32)> {
    if direction.1 == 0 {
        return vec![direction, (0,1), (0,-1)]
    } else {
        return vec![direction, (1,0), (-1,0)]
    }
}

fn char_matrix_to_string_matrix(char_matrix: &Vec<Vec<char>>) -> String {
    return char_matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
