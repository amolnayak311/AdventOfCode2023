use std::fs::read_to_string;
use std::collections::HashSet;

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

    let res = parsed_file.into_iter().map(|entries| {
        let (my_combinations, wining_combinations) = (entries.0, HashSet::<u32>::from_iter(entries.1.into_iter()));
        let mut score = 0;
        for my_num in my_combinations {
            if wining_combinations.contains(&my_num) {
                score += 1;
            }
        }
        if score > 0 {
            1 << (score - 1)
        } else {
            0
        }
    })
    .fold(0, |acc, x| acc + x);
    //.collect::<Vec<u32>>().sum();

    println!("{}", res);
}
