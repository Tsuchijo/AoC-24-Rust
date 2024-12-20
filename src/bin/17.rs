advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut reg_b = 0;
    let mut reg_c = 0;
    let lines = input.lines().collect::<Vec<&str>>();
    let mut reg_a = lines[0].split("Register A: ").skip(1).next().unwrap().parse::<u32>().unwrap();
    let instructions: Vec<u32> = lines[4][9..].split(",").map(|x| x.parse().unwrap()).collect();
    let mut program_counter = 0;
    let mut output: Vec<u32> = Vec::new();
    while program_counter < instructions.len() {
        match instructions[program_counter] {
            0 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_a = reg_a / 2_u32.pow(operand);
                program_counter += 2;
            }

            1 => {
                let operand = instructions[program_counter + 1];
                reg_b = reg_b ^ operand;
                program_counter += 2;
            }

            2 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_b = operand % 8;
                program_counter += 2;
            }

            3 => {
                let operand = instructions[program_counter + 1];
                if reg_a == 0 {
                    program_counter += 2;
                }
                else {
                    program_counter = operand as usize;
                }
            }

            4 => {
                reg_b = reg_b ^ reg_c;
                program_counter += 2;
            }

            5 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                output.push(operand % 8);
                program_counter += 2;
            }

            6 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_b = reg_b / 2_u32.pow(operand);
                program_counter += 2;
            }

            7 => {
                let operand = combo_operand(instructions[program_counter + 1], reg_a, reg_b, reg_c);
                reg_c = reg_c / 2_u32.pow(operand);
                program_counter += 2;
            }

            _ => {

            }
        }
    }
    let mut output_string = output.iter().map(|x| {
        let mut str = (*x).to_string();
        str.push(',');
        return str;
    }).collect::<String>();
    output_string.pop();
    println!("{:?}", output_string);
    Some(output_string)
}

fn combo_operand(operand: u32, reg_a: u32, reg_b: u32, reg_c: u32) -> u32 {
    return match operand {
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => operand
    }
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
        let test: String = "4,6,3,5,6,3,5,2,1,0".to_string();
        assert_eq!(result, Some(test));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
