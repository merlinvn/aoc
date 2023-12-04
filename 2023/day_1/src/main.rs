use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1() {
    let input_file = "./input.txt";
    let mut sum = 0;
    // read from the file line by line
    for line in std::fs::read_to_string(input_file)
        .expect("Failed to read file")
        .lines()
    {
        // get the first char that is a digit
        let first_char = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .expect("Failed to find digit");

        let last_char = line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .expect("Failed to find digit");

        // println!("{}-{}", first_char, last_char);

        // combine first and last char to get the number
        let number = format!("{}{}", first_char, last_char)
            .parse::<u32>()
            .expect("Failed to parse number");

        sum += number;
    }
    println!("Part1 result: {}", sum);
}

fn to_num(s: &str) -> Option<String> {
    match s {
        "one" => Some("1".to_owned()),
        "two" => Some("2".to_owned()),
        "three" => Some("3".to_owned()),
        "four" => Some("4".to_owned()),
        "five" => Some("5".to_owned()),
        "six" => Some("6".to_owned()),
        "seven" => Some("7".to_owned()),
        "eight" => Some("8".to_owned()),
        "nine" => Some("9".to_owned()),
        _ => None,
    }
}

fn part2() {
    let input_file = "./input.txt";
    let mut sum = 0;
    // read from the file line by line

    for line in std::fs::read_to_string(input_file)
        .expect("Failed to read file")
        .lines()
    {
        // find all the position occurence of "abc" in the string
        let digits = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        // for each char in the line check if it is a digit or begninning of a digit word
        // store that digit to first or second char of a number
        let mut first = Option::None;
        let mut second = Option::None;

        for char_index in 0..line.len() {
            let c = line.chars().nth(char_index).unwrap();
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = Some(c.to_string());
                    second = Some(c.to_string());
                } else {
                    second = Some(c.to_string());
                }
            } else {
                for digit in digits {
                    if line[char_index..].starts_with(digit) {
                        if first.is_none() {
                            first = to_num(digit);
                            second = to_num(digit);
                        } else {
                            second = to_num(digit);
                        }
                    }
                }
            }
        }

        // println!("{}-{}", first.as_ref().unwrap(), second.as_ref().unwrap());

        // combine first and last char to get the number
        let number = format!("{}{}", first.unwrap(), second.unwrap())
            .parse::<u32>()
            .expect("Failed to parse number");

        sum += number;
    }
    println!("Part2 result: {}", sum);
}

fn chat_gpt1() -> io::Result<()> {
    let path = "input.txt"; // Replace with the path to your input file
    let input = File::open(path)?;
    let buffered = io::BufReader::new(input);

    let mut total_sum = 0;

    for line in buffered.lines() {
        let line = line?;
        let calibration_value = extract_calibration_value(&line);
        total_sum += calibration_value;
    }

    println!("Total sum of calibration values: {}", total_sum);
    Ok(())
}

fn extract_calibration_value(line: &str) -> i32 {
    let first_digit = line.chars().find(|c| c.is_ascii_digit());
    let last_digit = line.chars().rev().find(|c| c.is_ascii_digit());

    match (first_digit, last_digit) {
        (Some(f), Some(l)) => {
            let number_str = format!("{}{}", f, l);
            number_str.parse::<i32>().unwrap_or(0)
        }
        _ => 0,
    }
}

use std::collections::HashMap;

fn chat_gpt2() -> io::Result<()> {
    let path = "input.txt"; // Replace with the path to your input file
    let input = File::open(path)?;
    let buffered = io::BufReader::new(input);

    let mut total_sum = 0;
    let digit_map = create_digit_map();

    for line in buffered.lines() {
        let line = line?;
        let calibration_value = extract_calibration_value2(&line, &digit_map);
        total_sum += calibration_value;
    }

    println!("Total sum of calibration values: {}", total_sum);
    Ok(())
}

fn create_digit_map() -> HashMap<String, char> {
    let mapping = vec![
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    mapping
        .into_iter()
        .map(|(s, c)| (s.to_string(), c))
        .collect()
}

fn extract_calibration_value2(line: &str, digit_map: &HashMap<String, char>) -> i32 {
    let mut modified_line = line.to_string();
    for (word, digit) in digit_map {
        modified_line = modified_line.replace(word, &digit.to_string());
    }

    let first_digit = modified_line.chars().find(|c| c.is_ascii_digit());
    let last_digit = modified_line.chars().rev().find(|c| c.is_ascii_digit());

    match (first_digit, last_digit) {
        (Some(f), Some(l)) => {
            let number_str = format!("{}{}", f, l);
            number_str.parse::<i32>().unwrap_or(0)
        }
        _ => 0,
    }
}

fn main() {
    part1();
    let _ = chat_gpt1();
    part2();
    let _ = chat_gpt2();
}
