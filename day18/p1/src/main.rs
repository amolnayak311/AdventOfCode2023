use std::fs::read_to_string;

// Taken from https://github.com/henryiii/aoc2023/blob/main/src/bin/18.rs
fn main() {
    let lines = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let splits = l.split(" ").collect::<Vec<&str>>();
            (splits[0].chars().next().unwrap(), splits[1].parse::<u8>().unwrap())
        }).collect::<Vec<(char, u8)>>();

    // Get bounds of the terrain
    // let (max_min_by_row, _, _) =
    //     lines.iter().fold((HashMap::<i32, (i32, i32)>::new(), 0, 0),
    //         |(mut max_min_by_row, curr_x, curr_y),
    //                               (direction, delta)| {
    //         let (new_x, new_y) = match *direction {
    //             'U'  => (curr_x,  curr_y - *delta as i32),
    //             'D'  => (curr_x,  curr_y + *delta as i32),
    //             'L'  => (curr_x - *delta as i32, curr_y),
    //             'R'  => (curr_x + *delta as i32, curr_y),
    //              _   => panic!("Unexpected direction")
    //         };
    //         (curr_y.min(new_y)..=new_y.max(curr_y)).for_each(|row_id| {
    //             match max_min_by_row.get(&row_id) {
    //                 Some((min_bound, max_bound))  =>
    //                     max_min_by_row.insert(row_id, (curr_x.min(new_x).min(*min_bound), curr_x.max(new_x).max(*max_bound))),
    //                 None                          =>
    //                     max_min_by_row.insert(row_id, (curr_x.min(new_x), curr_x.max(new_x)))
    //             };
    //         });
    //         (max_min_by_row, new_x, new_y)
    //     });
    // let total_area = max_min_by_row.iter().fold(0, |acc, (_, (min_bound, max_bound))| {
    //     acc + max_bound - min_bound + 1
    // });
    // // println!("{}", );
    // // println!("{}", );
    //
    // println!("{}", total_area);

    let (perimeter, area, _) = lines.iter().map(|(x, y)| (x, *y as i32))
        .fold((0, 0, (0, 0)), |(p, a, (y, x)), (d, l)| match d {
            'R' => (p + l, a, (y, x + l)),
            'L' => (p + l, a, (y, x - l)),
            'D' => (p + l, a + x * l, (y + l, x)),
            'U' => (p + l, a - x * l, (y - l, x)),
            _ => panic!("Got {d}, expected R, L, D, or U"),
        });
        println!("{}", area + perimeter / 2 + 1);
}
