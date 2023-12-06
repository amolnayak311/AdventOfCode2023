use std::fs::read_to_string;

fn get_file_as_grid() -> Vec<Vec<char>> {
    read_to_string("./src/input.txt").unwrap().lines().map(|x|x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

fn get_adj(x: usize, y: usize, nr: usize, nc: usize) -> Vec<(usize, usize)> {

    vec![(x, y + 1), (x + 1, y), (x, y - 1), (x - 1, y), (x + 1, y + 1), (x - 1 ,y - 1), (x + 1, y - 1), (x - 1, y + 1)]
    .into_iter()
    .filter(|x| x.0 < nr && x.1 < nc)
    .collect()
}

fn get_number_and_update_grid(ix: usize, iy: usize, grid: &mut Vec<Vec<char>>) -> u32 {
    if grid[ix][iy].is_digit(10) {
        // Naive approach, go left till you find digits
        let mut s_idx = iy;
        loop {
            if grid[ix][s_idx].is_digit(10) {
                if s_idx == 0 {
                    break
                } else {
                    s_idx -= 1;
                }
            } else {
                s_idx += 1;
                break;
            }
        }

        let mut e_idx = s_idx;
        while e_idx <  grid[ix].len() {
            if grid[ix][e_idx].is_digit(10) {
                e_idx += 1;
            } else {
                break;
            }
        }
        let res = String::from_iter((grid[ix][s_idx..e_idx]).to_vec()).parse::<u32>().unwrap();
        let mut idx = s_idx;
        while idx < e_idx {
            grid[ix][idx] = '.';
            idx += 1;
        }
        //println!("{}, {}, {}", sIdx, eIdx, res);
        res
    } else {
        0
    }
}

fn traverse_grid_and_compute(grid: &mut Vec<Vec<char>>) -> u32 {
    let mut res = 0;
    for ix in 0..grid.len() {
        for iy in 0..grid[ix].len() {
            let chr = grid[ix][iy];
            if chr != '.' && chr == '*' {
                let potential_adj_gears = get_adj(ix, iy, grid.len(), grid[ix].len())
                .iter()
                .map(|adj| get_number_and_update_grid(adj.0, adj.1, grid))
                .filter(|x|*x>0)
                .collect::<Vec<u32>>();
                if potential_adj_gears.len() == 2 {
                    res += potential_adj_gears[0] * potential_adj_gears[1];
                } 
            }
        }
    }

    res
}

fn main() {
    let mut grid = get_file_as_grid();
    //println!("{:?}, {}, {}", get_adj(139, 139, nr, nc), nr, nc);
    println!("{}", traverse_grid_and_compute(&mut grid));
}
