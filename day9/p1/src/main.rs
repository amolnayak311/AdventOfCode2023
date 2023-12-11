use std::fs::read_to_string;
fn find_interpolated(diffs: Vec<i32>) -> i32 {
    if diffs.iter().fold(true, |acc, x| acc && *x == 0) {
        0
    } else {
        let current_diffs = (1..diffs.len()).map(|x| {
            diffs[x] - diffs[x - 1]
        }).collect::<Vec<i32>>();
        current_diffs[current_diffs.len() - 1] + find_interpolated(current_diffs)
    }
}
fn main() {
    let res = read_to_string("./src/input.txt").unwrap().lines()
    .map(|x| x.split(" ").filter(|x| x.trim() != "").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>())
    .fold(0, |acc, measurements| 
        acc + measurements[measurements.len() - 1] + find_interpolated(measurements)
    );
    println!("{}", res);
}
