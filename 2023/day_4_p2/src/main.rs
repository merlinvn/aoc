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

    // a vect of scratch card numbers
    let mut scratch_cards = std::collections::HashMap::new();

    // for line in inputs {
    for line in std::fs::read_to_string(file_name).unwrap().lines() {
        // process card
        let mut card = line.split(':');
        let card_number = card
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

        println!("Match count: {}", match_count);

        scratch_cards.entry(card_number).or_insert(1_i32);

        let current_count = *scratch_cards.get(&card_number).unwrap();

        if match_count > 0 {
            for i in 1..match_count as i32 + 1 {
                println!("I: {}", i);
                let processing_card = card_number + i;
                scratch_cards.entry(processing_card).or_insert(1);
                let card_i_count = scratch_cards.get(&processing_card).unwrap();
                println!("Card i count: {}", card_i_count);
                println!("Current count: {}", current_count);
                scratch_cards.insert(card_number + i, card_i_count + current_count);
            }
        }

        // println!("Card number: {}", card_number);
        // println!("In hand: {:?}", in_hand);
        // println!("Winning number: {:?}", winning_number);
    }

    // sum all value in scratch_cards
    let sum = scratch_cards.values().sum::<i32>();
    println!("Number of scratch_cards: {}", sum);
}
