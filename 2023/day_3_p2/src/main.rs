use std::cmp::{max, min};

fn main() {
    // read schematic from file
    let file_name = "input.txt";
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in std::fs::read_to_string(file_name).unwrap().lines() {
        schematic.push(line.chars().collect());
    }
    // let mut schematic: Vec<Vec<char>> = vec![
    //     vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
    //     vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
    //     vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
    //     vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
    //     vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
    //     vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
    //     vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
    // ];

    let sum = sum_part_numbers(&mut schematic);
    println!("Sum of part numbers: {}", sum);
}

fn sum_part_numbers(schematic: &mut Vec<Vec<char>>) -> u32 {
    let mut sum = 0;
    let rows = schematic.len();
    let cols = schematic[0].len();

    for i in 0..rows {
        for j in 0..cols {
            // * is a gear
            if schematic[i][j] == '*' {
                let mut gear_parts = Vec::new();
                // Check if the gear is adjacent to a number
                for x in i.saturating_sub(1)..=min(i + 1, rows - 1) {
                    for y in j.saturating_sub(1)..=min(j + 1, cols - 1) {
                        // If it is, read the number and add it to the list of gears
                        if schematic[x][y].is_ascii_digit() {
                            // need to go all the way to the left in order to get the full number
                            let mut col = y;
                            while col > 0 && schematic[x][col - 1].is_ascii_digit() {
                                col -= 1;
                            }
                            let (number, length) = read_full_number(schematic, x, col);
                            mark_number_as_processed(schematic, x, col);
                            println!("number: {}, length: {}", number, length);
                            gear_parts.push(number);
                        }
                    }
                }

                // if leng of list of gears is eaxtly 2, add the product of the two gears to the sum
                if gear_parts.len() == 2 {
                    sum += gear_parts[0] * gear_parts[1];
                }
                // println!("i: {}, j: {}", i, j);
            }
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
