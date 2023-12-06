fn main() {
    // // read schematic from file
    let file_name = "input.txt";
    // let inputs = vec![
    //     "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
    //     "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
    //     "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
    //     "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
    //     "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
    //     "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    // ];

    let mut sum = 0;

    // for line in inputs {
    for line in std::fs::read_to_string(file_name).unwrap().lines() {
        // process card
        let mut card = line.split(':');
        let _card_number = card
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        // process in hand and winning number
        let mut cards = card.next().unwrap().trim().split('|');

        let in_hand = cards
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        // winning number may be empty

        let winning_number = cards
            .next()
            .unwrap_or("")
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let match_count = in_hand
            .iter()
            .filter(|x| winning_number.contains(x))
            .count();

        //sum += 2 ^ (match_count - 1);

        println!("Match count: {}", match_count);
        match match_count {
            0 => sum += 0,
            c => sum += 2_i32.pow((c - 1) as u32),
        }

        // println!("Card number: {}", card_number);
        // println!("In hand: {:?}", in_hand);
        // println!("Winning number: {:?}", winning_number);
    }
    println!("Sum: {}", sum);
}
