use std::fs::read_to_string;
use std::collections::HashSet;
use std::collections::HashMap;

fn parse_file() -> Vec<(Vec<u32>, Vec<u32>)> {
    read_to_string("./src/input.txt")
    .unwrap()
    .lines()
    .map(|line| {
        let colon_index = line.find(':').unwrap();
        let partition = &line[(colon_index + 1)..line.len()];
        let parsed_numeric_splits = partition
        .split('|')
        .map(|splt| {
            splt.split(' ')
            .filter(|x|x.trim() != "")
            .map(|num|num.parse::<u32>().unwrap()).collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
        (parsed_numeric_splits[0].clone(), parsed_numeric_splits[1].clone())
    }).collect::<Vec<(Vec<u32>, Vec<u32>)>>()
}
fn main() {

    let parsed_file = parse_file();
    //println!("{:?}", parsed_file);
    let mut map = HashMap::new();
    let num_cards = parsed_file.len();
    for card in 1..=num_cards {
        map.insert(card, 1);
    }
    
    let mut card_no = 1;
    for line in parsed_file {
        let (my_combinations, wining_combinations) = (line.0, HashSet::<u32>::from_iter(line.1.into_iter()));
        let mut score = 0;
        for my_num in my_combinations {
            if wining_combinations.contains(&my_num) {
                score += 1;
            }
        }
        let current_card_copies = map.get_mut(&card_no).unwrap().clone();
        for winning_cards in (card_no + 1)..=(card_no + score) {
            let winning_card_current = map.get(&winning_cards).unwrap().clone();
            map.insert(winning_cards,  winning_card_current + current_card_copies);
        }
        //println!("{}", score);
        card_no += 1;

    }
    let mut res = 0;
    for card in 1..=num_cards {
        res += map.get(&card).unwrap().clone();
    }
    println!("{:?}", res);
    
}
