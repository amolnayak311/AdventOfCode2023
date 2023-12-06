use std::fs::read_to_string;

fn main() {
    let sum_of_games = read_to_string("./src/input.txt")
    .unwrap()
    .lines()
    .map(String::from)
    .map(|x| {
        let splits = x.split(":").collect::<Vec<&str>>();
        let game_id = splits[0].get(5..).unwrap().parse::<u32>().unwrap();
        let all_games_possible: bool = splits[1].split(";")
                        .map(|s| {
                            let splits = s.trim().split(" ").map(|c|c.replace(",", "")).collect::<Vec<String>>();

                            (0..splits.len())
                            .fold((0, 0, 0), |acc, x| {                                
                                if splits[x] == "red" {
                                    (splits[x - 1].parse::<u32>().unwrap(), acc.1, acc.2)
                                } else if splits[x] == "green" {
                                    (acc.0, splits[x - 1].parse::<u32>().unwrap(), acc.2)
                                } else if splits[x] == "blue" {
                                    (acc.0, acc.1, splits[x - 1].parse::<u32>().unwrap())
                                } else {
                                    acc
                                }
                            })
                        })
                        .fold(true, |acc, game| acc && (game.0 <= 12 && game.1 <= 13 && game.2 <= 14));
                        
        (game_id, all_games_possible)
    }
    )
    .filter(|x|x.1)
    .map(|x|x.0).fold(0, |acc, x| acc + x);

    println!("{}", sum_of_games);
}
