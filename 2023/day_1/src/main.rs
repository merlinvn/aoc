// fn part1() {
//     let input_file = "./input.txt";
//     let mut sum = 0;
//     // read from the file line by line
//     for line in std::fs::read_to_string(input_file)
//         .expect("Failed to read file")
//         .lines()
//     {
//         // get the first char that is a digit
//         let first_char = line
//             .chars()
//             .find(|c| c.is_ascii_digit())
//             .expect("Failed to find digit");
//
//         let last_char = line
//             .chars()
//             .rev()
//             .find(|c| c.is_ascii_digit())
//             .expect("Failed to find digit");
//
//         println!("{}-{}", first_char, last_char);
//
//         // combine first and last char to get the number
//         let number = format!("{}{}", first_char, last_char)
//             .parse::<u32>()
//             .expect("Failed to parse number");
//
//         sum += number;
//     }
//     println!("{}", sum);
// }

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

        println!("{}-{}", first.as_ref().unwrap(), second.as_ref().unwrap());

        // combine first and last char to get the number
        let number = format!("{}{}", first.unwrap(), second.unwrap())
            .parse::<u32>()
            .expect("Failed to parse number");

        sum += number;
    }
    println!("{}", sum);
}

fn main() {
    part2();
}
