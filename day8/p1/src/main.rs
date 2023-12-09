use std::fs::read_to_string;
use std::collections::HashMap;
fn traverse<'a>(pattern: &'a str, start: &'a str, map: &'a HashMap<String, (String, String)>) -> &'a str {
    let mut current = start; 
    for c in pattern.chars() {
        let potential_next = map.get(current).unwrap();
        if c == 'L' {
            current = &potential_next.0;
        } else {
            current = &potential_next.1;
        }
    }
    current
}
fn main() {
    let file_unwrapped = read_to_string("./src/input.txt").unwrap();
    let lines = file_unwrapped.lines().filter(|line| *line != "").collect::<Vec<&str>>();
    let pattern = lines[0];

    let hm: HashMap<String, (String, String)> = lines[1..].into_iter().fold(HashMap::new(), |mut hm, line| {
        let (key, lhs, rhs) = (line[0..3].to_string(), line[7..10].to_string(), line[12..15].to_string());
        hm.insert(key, (lhs, rhs));
        hm
    });
    let mut count = 0;
    let mut current = "AAA";
    loop {
        count += 1;
        current = traverse(&pattern, current, &hm);
        if current == "ZZZ" {
            break;
        }
    }

    println!("{:?}", count * pattern.len());

    
}
