use std::fs::read_to_string;

fn main() {
    let sum_of_games = read_to_string("./src/input.txt")
    .unwrap()
    .lines()
    .map(String::from)
    .map(|x| {
        let splits = x.split(":").collect::<Vec<&str>>();
        let min_cubes = splits[1].split(";")
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
                        .fold((0, 0, 0), |acc, game| (acc.0.max(game.0), acc.1.max(game.1), acc.2.max(game.2)));
                        
        min_cubes.0 * min_cubes.1 * min_cubes.2
    }).fold(0, |acc, x| acc + x);

    println!("{}", sum_of_games);
}
