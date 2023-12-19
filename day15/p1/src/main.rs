use std::fs::read_to_string;

fn hash(str: &str) -> u8 {
    str.chars().fold(0, |acc, chr| {
        let temp_acc = 17 * (acc as u32 + chr as u32);
        (temp_acc % 256) as u8
    })
}
fn main() {

    let res = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .fold(0, |_, x| x.split(",").map(hash)
            .fold(0u32, |acc, num| acc + num as u32));
    println!("{}", res);
}
