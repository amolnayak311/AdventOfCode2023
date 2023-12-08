use std::fs::read_to_string;

fn main() {
    let parsed_lines = read_to_string("./src/input.txt").unwrap().lines()
                        .map(|x|
                            x[9..x.len()].replace(" ", "").trim().parse::<u64>().unwrap()
                        )
                        .collect::<Vec<u64>>();
    let(x, y) = (parsed_lines[0], parsed_lines[1]);
    
    let res = (0..=x).fold(0, |curr_matches, speed_time| {
            if speed_time * (x - speed_time) >  y {
                curr_matches + 1
            } else {
                curr_matches
            }
        });
    println!("{}", res);


}
