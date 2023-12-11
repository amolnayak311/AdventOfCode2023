use std::fs::read_to_string;
use std::collections::HashSet;
fn get_empty_rows_and_column_idx(map: &Vec<Vec<char>>) -> (HashSet<usize>, HashSet<usize>) {
    // in efficient 2 pass approach
    let empty_rows = (0..map.len()).fold(HashSet::new(), |mut empty_rows, row_idx| {
        if map[row_idx].iter().fold(true, |state, chr| state && *chr == '.') {
            empty_rows.insert(row_idx);
        }
        empty_rows
    });
    let empty_cols = (0..map[0].len()).fold(HashSet::new(), |mut empty_cols, col_idx| {
        if map.iter().fold(true, |state, vec| state && vec[col_idx] == '.') {
            empty_cols.insert(col_idx);
        }
        empty_cols
    });
    (empty_rows, empty_cols)
}

fn get_distance(sx: usize, sy:usize, ex:usize, ey:usize, empty_rows: &HashSet<usize>, empty_cols: &HashSet<usize>) -> usize {
    let extra_rows = (sx.min(ex)..ex.max(sx)).fold(0usize, |acc, idx| {
        if empty_rows.contains(&idx) {
            acc + 1
        } else {
            acc
        }
    });
    let extra_cols = (sy.min(ey)..ey.max(sy)).fold(0usize, |acc, idx| {
        if empty_cols.contains(&idx) {
            acc + 1
        } else {
            acc
        }
    });
    ex.max(sx) - ex.min(sx) + extra_rows + ey.max(sy) - sy.min(ey)+ extra_cols
}
fn main() {
    let map = read_to_string("./src/input.txt")
    .unwrap()
    .lines()
    .map(|x| x.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();
    let (empty_rows, empty_cols) = get_empty_rows_and_column_idx(&map);

    let galaxies = (0..map.len()).flat_map(|x_idx| {
        (0..map[x_idx].len()).fold(vec![],|mut vec, y_idx| {
            if map[x_idx][y_idx] == '#' {
                vec.push((x_idx, y_idx))
            }
            vec
        })
    }).collect::<Vec<(usize, usize)>>();
    
    let res = (0..galaxies.len()).fold(0, |acc, idx1| 
        acc +  ((idx1 + 1)..galaxies.len()).fold(0, |acc1, idx2| 
            acc1 + get_distance(galaxies[idx1].0, galaxies[idx1].1, galaxies[idx2].0, galaxies[idx2].1, &empty_rows, &empty_cols)
        )
    );
    // println!("{:?}", get_distance(0, 3, 8, 7, &empty_rows, &empty_cols));
    // println!("{:?}", get_distance(2, 0, 6, 9, &empty_rows, &empty_cols));
    println!("{}", res);


}
