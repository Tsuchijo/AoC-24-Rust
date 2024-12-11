use std::collections::HashSet;
use std::vec;
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let num_matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let width = num_matrix[0].len();
    let height = num_matrix.len();

    let mut zero_locations: Vec<(usize, usize)> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if num_matrix[y][x] == 0 {
                zero_locations.push((x, y));
            }
        }
    }

    let trail_heads: Vec<u32> = zero_locations
        .iter()
        .map(|(x, y)| {
            //println!("{:?}", get_trailheads(&num_matrix, *x, *y, 0));
            get_trailheads(&num_matrix, *x, *y, 0).len() as u32
        })
        .collect();
    // println!("{:?}", zero_locations);
    // println!("{:?}", trail_heads);
    return Some(trail_heads.iter().sum());
}

fn get_trailheads(
    num_matrx: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    value: u32,
) -> HashSet<(u32, u32)> {
    let mut trailheads: HashSet<(u32, u32)> = HashSet::new();
    for (i,j) in vec![(0, -1), (0, 1), (1, 0), (-1, 0)] {
        let x_adj = x as i32 + i;
        let y_adj = y as i32 + j;
        if x_adj < 0
            || x_adj >= num_matrx[0].len() as i32
            || y_adj < 0
            || y_adj >= num_matrx.len() as i32
        {
            continue;
        }
        let adj_value = num_matrx[y_adj as usize][x_adj as usize];
        if adj_value == value + 1 && adj_value < 9 {
            trailheads.extend(&get_trailheads(
                num_matrx,
                x_adj as usize,
                y_adj as usize,
                adj_value,
            ));
        } else if adj_value == value + 1 && adj_value == 9 {
            trailheads.insert((x_adj as u32, y_adj as u32));
        }
    }
    return trailheads;
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_matrix: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let width = num_matrix[0].len();
    let height = num_matrix.len();

    let mut zero_locations: Vec<(usize, usize)> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if num_matrix[y][x] == 0 {
                zero_locations.push((x, y));
            }
        }
    }

    let trail_heads: Vec<u32> = zero_locations
        .iter()
        .map(|(x, y)| {
            //println!("{:?}", get_trailheads(&num_matrix, *x, *y, 0));
            get_trail_rating(&num_matrix, *x, *y, 0)
        })
        .collect();
    // println!("{:?}", zero_locations);
    //println!("{:?}", trail_heads);
    return Some(trail_heads.iter().sum());
}

fn get_trail_rating(num_matrx: &Vec<Vec<u32>>, x: usize, y: usize, value: u32) -> u32 {
    let mut trail_rating = 0;
    for k in vec![-1, 1] {
        for d in vec![-1, 1] {
            let i: i32 = ((k == d) as i32) * k;
            let j = ((k != d) as i32) * k;
            let x_adj = x as i32 + i;
            let y_adj = y as i32 + j;
            if x_adj < 0
                || x_adj >= num_matrx[0].len() as i32
                || y_adj < 0
                || y_adj >= num_matrx.len() as i32
            {
                continue;
            }
            let adj_value = num_matrx[y_adj as usize][x_adj as usize];
            if adj_value == value + 1 && adj_value < 9 {
                trail_rating +=
                    get_trail_rating(num_matrx, x_adj as usize, y_adj as usize, adj_value);
            } else if adj_value == value + 1 && adj_value == 9 {
                trail_rating += 1;
            }
        }
    }
    return trail_rating;
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
