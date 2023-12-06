use std::cmp::{max, min};

fn main() {
    // read schematic from file
    let file_name = "input.txt";
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in std::fs::read_to_string(file_name).unwrap().lines() {
        schematic.push(line.chars().collect());
    }

    let sum = sum_part_numbers(&mut schematic);
    println!("Sum of part numbers: {}", sum);
}

fn sum_part_numbers(schematic: &mut Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    let rows = schematic.len();
    let cols = schematic[0].len();

    for i in 0..rows {
        for j in 0..cols {
            if schematic[i][j].is_ascii_digit() {
                //read full number
                let number = read_full_number(schematic, i, j);
                println!("number: {}", number.0);

                // check if adjacent to symbol
                if is_adjacent_to_symbol(schematic, i, j, number.1) {
                    sum += number.0 as i32;
                }

                // mack number as processed
                mark_number_as_processed(schematic, i, j);
            }
            // match schematic[i][j].is_ascii_digit() {
            //     true => {
            //         if is_adjacent_to_symbol(schematic, i, j) {
            //             sum += read_full_number(schematic, i, j) as i32;
            //             // Mark the number as processed to avoid double counting
            //             mark_number_as_processed(schematic, i, j);
            //         }
            //     }
            //     false => (),
            // }
        }
    }

    sum
}

fn is_adjacent_to_symbol(schematic: &Vec<Vec<char>>, i: usize, j: usize, length: usize) -> bool {
    //for row from max(i-1,  0) to min(i+1, schematic.len())
    //for col from max(j-1,  0) to min(j+1, schematic[0].len())

    for row in i.saturating_sub(1)..=min(i + 1, schematic.len() - 1) {
        println!("row: {}", row);
        for col in j.saturating_sub(1)..=min(j + length, schematic[0].len() - 1) {
            println!("col: {}", col);
            if schematic[row][col] != '.' && !schematic[row][col].is_ascii_digit() {
                return true;
            }
        }
    }

    false
}

fn read_full_number(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> (u32, usize) {
    let mut number = String::new();
    let mut col = j;

    while col < schematic[0].len() && schematic[i][col].is_ascii_digit() {
        number.push(schematic[i][col]);
        col += 1;
    }

    (number.parse::<u32>().unwrap_or(0), number.len())
}

fn mark_number_as_processed(schematic: &mut Vec<Vec<char>>, i: usize, j: usize) {
    let mut col = j;
    while col < schematic[0].len() && schematic[i][col].is_ascii_digit() {
        schematic[i][col] = '.';
        col += 1;
    }
}
