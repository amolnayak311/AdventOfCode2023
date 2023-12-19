use std::fs::read_to_string;
fn main() {
    let  file_handle = read_to_string("./src/input.txt").unwrap();
    let lines = file_handle
                            .lines()
                            .map(|line| line.chars().collect::<Vec<char>>())
                            .collect::<Vec<Vec<char>>>();

    let (nr, nc) = (lines.len(), lines[0].len());
    let res = (0..nc).map(|col_idx| {
        // go through all the rows in that column and find the stone index
        let (stone_idx, _) = (0..nr).fold((vec![], 0),
            |(mut stone_idx, next_stone_idx), row_idx| {
                let chr = lines[row_idx][col_idx];
                if chr == 'O' {
                    stone_idx.push(next_stone_idx);
                    (stone_idx, next_stone_idx + 1)
                } else if chr == '#'{
                    (stone_idx, row_idx + 1)
                } else {
                    (stone_idx, next_stone_idx )
                }
            }
        );
        stone_idx
    }).flat_map(|x| x)
        .map(|x| nr - x).sum::<usize>();

    println!("{}", res);

}
