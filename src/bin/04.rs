use std::char;

use regex::Regex;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let re_xmas = Regex::new(r"(XMAS)").unwrap();
    let re_samx = Regex::new(r"(SAMX)").unwrap();
    let char_matrix: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let char_matrix_t: Vec<Vec<char>> = transpose_char_matrix(char_matrix.clone());
    // rotate 45 degrees
    let diag_matrix: Vec<Vec<char>> = rotate_45_degrees(&char_matrix);
    let diag_matrix_t: Vec<Vec<char>> = rotate_neg_45_degrees(&char_matrix_t);
    let mut result: u32 = 0;
    result += re_xmas.captures_iter(&char_matrix_to_string_matrix(&char_matrix)).count() as u32;
    result += re_xmas.captures_iter(&char_matrix_to_string_matrix(&char_matrix_t)).count() as u32;
    result += re_xmas.captures_iter(&char_matrix_to_string_matrix(&diag_matrix)).count() as u32;
    result += re_xmas.captures_iter(&char_matrix_to_string_matrix(&diag_matrix_t)).count() as u32;
    result += re_samx.captures_iter(&char_matrix_to_string_matrix(&char_matrix)).count() as u32;
    result += re_samx.captures_iter(&char_matrix_to_string_matrix(&char_matrix_t)).count() as u32;
    result += re_samx.captures_iter(&char_matrix_to_string_matrix(&diag_matrix)).count() as u32;
    result += re_samx.captures_iter(&char_matrix_to_string_matrix(&diag_matrix_t)).count() as u32;
    return Some(result);
}


fn rotate_45_degrees(char_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let cols: i32 = char_matrix.len() as i32;
    let rows: i32 = char_matrix[0].len() as i32;
    let max_cols_rows = cols.max(rows);
    let diag_matrix: Vec<Vec<char>> = (-max_cols_rows..max_cols_rows*2)
        .map(|d| {
            (0..max_cols_rows+2)
                .map(|row| {
                    let i = d + row;
                    let j = row;
                    if i >= 0 && i < cols as i32 && j >= 0 && j < rows as i32 {
                        char_matrix[i as usize][j as usize]
                    } else {
                        ' '
                    }
                })
                .collect()
        })
        .collect();
    return diag_matrix;
}

fn rotate_neg_45_degrees(char_matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let cols: i32 = char_matrix.len() as i32;
    let rows: i32 = char_matrix[0].len() as i32;
    let max_cols_rows = cols.max(rows);
    let diag_matrix: Vec<Vec<char>> = (-max_cols_rows..max_cols_rows*2)
        .map(|d| {
            (0..max_cols_rows+2)
                .map(|row| {
                    let i = d - row;
                    let j = row;
                    if i >= 0 && i < cols as i32 && j >= 0 && j < rows as i32 {
                        char_matrix[i as usize][j as usize]
                    } else {
                        ' '
                    }
                })
                .collect()
        })
        .collect();
    return diag_matrix;
}

fn transpose_char_matrix(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = v.len();
    let cols = v[0].len();
    let transposed: Vec<Vec<_>> = (0..cols).map(|col| {
        (0..rows)
            .map(|row| v[row][col])
            .collect()
    }).collect();
    return transposed;
}

pub fn char_matrix_to_string_matrix(char_matrix: &Vec<Vec<char>>) -> String {
    char_matrix
        .iter()
        .map(|v| v.iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_matrix: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut result: u32 = 0;
    let re_mas_sam = Regex::new(r"(MAS)|(SAM)").unwrap();
    for i in 0..char_matrix.len() {
        for j in 0..char_matrix[i].len() {
            if char_matrix[i][j] == 'A' {
                let strings: Vec<String> = diag_strings(&char_matrix, i as i32, j as i32);
                result += strings.iter().fold(true, |acc, s| acc && re_mas_sam.is_match(s)) as u32;
            }
        }
    }
    return Some(result);
}

fn diag_strings(char_matrix: &Vec<Vec<char>>, i_start: i32, j_start:i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let rows: i32 = char_matrix.len() as i32;
    let cols = char_matrix[0].len() as i32;
    for diag_mult in [-1, 1]{
        let mut diag_strings: Vec<char> = Vec::new();
        for d in (-1)..(2){
            let i = i_start + (diag_mult * d);
            let j = j_start + d;
            if i >= 0 && i < rows as i32 && j >= 0 && j < cols {
                diag_strings.push(char_matrix[i as usize][j as usize]);
            }
        }
        result.push(diag_strings.iter().collect());
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        //let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        //let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
