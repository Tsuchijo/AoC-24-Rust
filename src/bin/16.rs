use std::{collections::{HashMap, HashSet, VecDeque}, rc::{self, Rc}};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let maze: Vec<Vec<char>> = input.lines().map(|s: &str| s.chars().collect()).collect();
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
    let low_score = bfs_search(&maze, start_pos, end_pos)[0].score;
    return Some(low_score);
}

#[derive(Clone)]
#[derive(Eq, Hash, PartialEq, Debug)]
struct PathTree {
    score: u32,
    coordinate: (usize, usize),
    direction: (i32, i32),
    parent: Option<Rc<PathTree>>
}

fn bfs_search(grid: &Vec<Vec<char>>, start_coord: (usize, usize), end_coord: (usize, usize)) -> Vec<Rc<PathTree>> {
    let mut to_search: VecDeque<PathTree> = VecDeque::new();
    let mut end_paths: Vec<Rc<PathTree>> = Vec::new();
    let mut searched: HashMap<((usize, usize), (i32, i32)), u32> = HashMap::new();
    let mut lowest_score = 0xFFFFFFFF;
    to_search.push_back(PathTree{
        score: 0,
        coordinate: start_coord,
        direction: (1,0),
        parent: None
    });
    while !to_search.is_empty() {
        let coord_tree = Rc::new(to_search.pop_front().unwrap());
        for dir in get_rotations(coord_tree.direction) {
            let x = coord_tree.coordinate.0 as i32 + dir.0;
            let y = coord_tree.coordinate.1 as i32 + dir.1;
            if grid[y as usize][x as usize ] != '#' {
                let new_coord = (x as usize, y as usize);
                let mut new_path = PathTree{
                    score: coord_tree.score + 1,
                    coordinate: (x as usize, y as usize),
                    direction: dir,
                    parent: Some(coord_tree.clone())
                };

                if searched.contains_key(&(new_coord, dir)) {
                    if searched[&(new_coord, dir)] < new_path.score {
                        continue;
                    }
                }

                let mut turned = false;
                if dir != coord_tree.direction {
                    new_path.score += 1000;
                    turned = true;
                }
                if new_coord == end_coord {
                    if new_path.score <= lowest_score {
                        lowest_score = new_path.score;
                        end_paths.push(Rc::new(new_path));
                    }

                }
                else{
                    if new_path.score < lowest_score && turned {
                        to_search.push_back(new_path);
                    }
                    else if new_path.score < lowest_score {
                        to_search.push_front(new_path);
                    }
                }
            }
        }
        searched.insert((coord_tree.coordinate, coord_tree.direction), coord_tree.score);
    }
    end_paths.retain(|x| x.score == lowest_score);
    return end_paths;
}

fn get_rotations(direction: (i32, i32)) -> Vec<(i32, i32)> {
    if direction.1 == 0 {
        return vec![direction, (0,1), (0,-1)]
    } else {
        return vec![direction, (1,0), (-1,0)]
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze: Vec<Vec<char>> = input.lines().map(|s: &str| s.chars().collect()).collect();
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
    let optimal_paths = bfs_search(&maze, start_pos, end_pos);
    let mut optimal_tiles: HashSet<(u32, u32)> = HashSet::new();
    for mut path in optimal_paths {
        while path.parent.is_some() {
            optimal_tiles.insert((path.coordinate.0 as u32, path.coordinate.1 as u32));
            path = path.parent.clone().unwrap();
        }
    }
    return Some(optimal_tiles.len() as u32 + 1);
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
        assert_eq!(result, Some(64));
    }
}
