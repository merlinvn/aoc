use std::{
    fs::File,
    io::{self, BufRead},
};

struct Game {
    id: u32,
    record: Vec<(u32, u32, u32)>,
}

fn get_game_records(record: &str) -> Vec<(u32, u32, u32)> {
    let mut game_records = Vec::new();
    let re = regex::Regex::new(r"(([^;]*)[; ]*)").unwrap();
    for cap in re.captures_iter(record) {
        let single_record = cap.get(2).unwrap().as_str();

        let re = regex::Regex::new(r"(\d+) (red|green|blue)").unwrap();
        let mut single_play = (0, 0, 0);
        let plays = re.captures_iter(single_record).map(|cap| {
            let number = cap.get(1).unwrap().as_str().parse::<u32>().unwrap_or(0);
            let color = cap.get(2).unwrap().as_str();
            (number, color)
        });

        for play in plays {
            match play {
                (number, "red") => single_play.0 = number,
                (number, "green") => single_play.1 = number,
                (number, "blue") => single_play.2 = number,
                _ => (),
            }
        }
        game_records.push(single_play);
    }
    game_records
}

fn get_game(line: &str) -> Option<Game> {
    // use regex to get the id and record
    // Game (\d+): (.*)

    let re = regex::Regex::new(r"Game (\d+): (.*)").unwrap();

    let caps = re.captures(line);

    let game = match caps {
        Some(caps) => {
            let mut game = Game {
                id: 0,
                record: Vec::new(),
            };
            let id = caps.get(1).unwrap().as_str();
            let record = caps.get(2).unwrap().as_str();
            println!("id: {}, record: {}", id, record);

            let game_records = get_game_records(record);

            game.id = id.parse::<u32>().unwrap();
            game.record = game_records;

            Some(game)
        }
        None => None,
    };
    game
}

fn get_power(game: &Game) -> u32 {
    let max_red = game.record.iter().map(|(r, _, _)| r).max().unwrap_or(&1);
    let max_green = game.record.iter().map(|(_, g, _)| g).max().unwrap_or(&1);
    let max_blue = game.record.iter().map(|(_, _, b)| b).max().unwrap_or(&1);

    max_red * max_green * max_blue
}

fn main() {
    let path = "input.txt"; // Replace with the path to your input file
    let input = File::open(path).unwrap();
    let buffered = io::BufReader::new(input);

    let mut total_sum1 = 0;
    let mut total_sum2 = 0;

    for line in buffered.lines() {
        let line = line.unwrap();

        let game = get_game(&line).unwrap();

        // check if each record has less  than
        // 12 red cubes, 13 green cubes, and 14 blue cubes
        let valid = game
            .record
            .iter()
            .all(|(r, g, b)| *r <= 12 && *g <= 13 && *b <= 14);

        if valid {
            total_sum1 += game.id;
        }
        //

        let power = get_power(&game);
        total_sum2 += power;
    }

    println!("Total sum of game ids: {}", total_sum2);
}
