use std::fs::read_to_string;

fn main() {
    let parsed_lines = read_to_string("./src/input.txt").unwrap().lines()
                        .map(|x|
                            x[9..x.len()].split(' ').filter(|s| s.trim() != "").map(|p| p.parse::<u32>().unwrap()).collect::<Vec<u32>>()
                        )
                        .collect::<Vec<Vec<u32>>>();
    let res = parsed_lines[0].iter().zip(parsed_lines[1].iter()).map(|(x, y)| {
        (0..=*x).fold(0, |curr_matches, speed_time| {
            if speed_time * (x - speed_time) >  *y {
                curr_matches + 1
            } else {
                curr_matches
            }
        })
        
    }).product::<u32>();
    println!("{}", res);


}
