use std::fs::read_to_string;

fn main() {
    let patterns: Vec<(&str, u8)> = 
        vec![("one", 1), ("two", 2), ("three", 3),
             ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9), ("zero", 0),
             ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9), ("0", 0)];
    let lines: Vec<String> = read_to_string("./src/input.txt").unwrap().lines().map(String::from).collect();

    let res: u32 = lines.iter().map(|string| {
        let first_matched:Vec<(usize, u8)> = 
        patterns.iter()
        .map(|pat| (string.find(pat.0), pat.1))
        .filter(|x|x.0.is_some())
        .map(|x|(x.0.unwrap(), x.1)).collect();

        let last_matched:Vec<(usize, u8)> = 
        patterns.iter()
        .map(|pat| (string.rfind(pat.0), pat.1))
        .filter(|x|x.0.is_some())
        .map(|x|(x.0.unwrap(), x.1)).collect();

        //first_matched.sort_by(|a, b| a.0.cmp(&b.0));

        //last_matched.sort_by(|a, b| b.0.cmp(&a.0));

        let (_, first) = 
            first_matched.iter().fold((1000 as usize, 0 as u8), |acc, x| {
                let (index, _) = (acc.0, acc.1);
                if index > x.0 {
                    *x
                } else {
                    acc
                }
            });
        let (_, last) = 
            last_matched.iter().fold((0 as usize, 0 as u8), |acc, x| {
                let (index, _) = (acc.0, acc.1);
                if index <= x.0 {
                    *x
                } else {
                    acc
                }
            });
        (first * 10 + last) as u32
    }).sum();
    println!("{}", res);
}
