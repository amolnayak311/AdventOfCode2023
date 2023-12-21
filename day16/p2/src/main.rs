use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(PartialEq, Hash, Eq, Copy, Clone)]
enum Direction {
    EAST, WEST, NORTH, SOUTH,
}
fn traverse(visited: &mut HashSet<(usize, usize, Direction)>,
            ix: usize,
            iy: usize,
            direction: Direction,
            map: &Vec<Vec<char>>) -> () {


    if ix < map.len() && iy < map[0].len() && !visited.contains(&(ix, iy, direction)) {
        visited.insert((ix, iy, direction));
        match direction {
            Direction::EAST    => match map[ix][iy] {
                                     '.'   => if iy > 0 {
                                                traverse(visited, ix, iy - 1, direction, map)
                                              },
                                     '|'   => {
                                         traverse(visited, ix + 1, iy, Direction::NORTH, map);
                                         if ix > 0 {
                                             traverse(visited, ix - 1, iy, Direction::SOUTH, map);
                                         }
                                     },
                                     '-'   => if iy > 0 {
                                                 traverse(visited, ix, iy - 1, direction, map)
                                               },
                                     '\\'   => if ix > 0 {
                                                traverse(visited, ix - 1, iy, Direction::SOUTH, map)
                                               },
                                     '/'   => traverse(visited, ix + 1, iy, Direction::NORTH, map),
                                      _    => (),
                                },
            Direction::WEST    => match map[ix][iy] {
                                    '.'   => traverse(visited, ix, iy + 1, direction, map),
                                    '|'   => {
                                        traverse(visited, ix + 1, iy, Direction::NORTH, map);
                                        if ix > 0 {
                                            traverse(visited, ix - 1, iy, Direction::SOUTH, map);
                                        }
                                    },
                                    '-'   => traverse(visited, ix, iy + 1, direction, map),
                                    '\\'   => traverse(visited, ix + 1, iy, Direction::NORTH, map),
                                    '/'   => if ix > 0 {
                                                traverse(visited, ix - 1, iy, Direction::SOUTH, map);
                                             },
                                    _    => (),
                                },
            Direction::NORTH   => match map[ix][iy] {
                                    '.'   => traverse(visited, ix + 1, iy , direction, map),
                                    '|'   => traverse(visited, ix + 1, iy , direction, map),
                                    '-'   => {
                                                traverse(visited, ix, iy + 1 , Direction::WEST, map);
                                                if iy > 0 {
                                                    traverse(visited, ix, iy - 1 , Direction::EAST, map);
                                                }
                                            },
                                    '\\'   => traverse(visited, ix, iy + 1 , Direction::WEST, map),
                                    '/'   => if iy > 0 {
                                               traverse(visited, ix, iy - 1 , Direction::EAST, map);
                                            },
                                    _    => (),
                                },
            Direction::SOUTH   => match map[ix][iy] {
                                    '.'   => if ix > 0 {
                                                traverse(visited, ix - 1, iy , direction, map)
                                             },
                                    '|'   => if ix > 0 {
                                                 traverse(visited, ix - 1, iy, direction, map)
                                             },
                                    '-'   => {
                                               traverse(visited, ix, iy + 1 , Direction::WEST, map);
                                                if iy > 0 {
                                                    traverse(visited, ix, iy - 1 , Direction::EAST, map);
                                                }
                                            },
                                    '\\'   =>  if iy > 0 {
                                                traverse(visited, ix, iy - 1 , Direction::EAST, map);
                                                },
                                    '/'   => traverse(visited, ix, iy + 1 , Direction::WEST, map),
                                    _    => (),
                                },
        }


    }

}

fn main() {
    let map = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let res =
        vec![Direction::EAST, Direction::WEST, Direction::SOUTH, Direction::NORTH].iter()
            .flat_map(|direction| {
                match direction {
                    // Right to left
                    Direction::EAST  =>
                        (0..map.len()).map(|row| (row, map[0].len() - 1, *direction))
                            .collect::<Vec<(usize, usize, Direction)>>()
                    ,
                    // Left to right
                    Direction::WEST  =>
                        (0..map.len()).map(|row| (row, 0, *direction))
                            .collect::<Vec<(usize, usize, Direction)>>()
                    ,
                    // Top to bottom
                    Direction::NORTH  => (0..map[0].len()).map(|col| (0, col, *direction))
                            .collect::<Vec<(usize, usize, Direction)>>()
                    ,
                    // Bottom to top
                    Direction::SOUTH  => {
                        let nr = map.len();
                        (0..map[0].len()).map(|col| (nr - 1, col, *direction))
                            .collect::<Vec<(usize, usize, Direction)>>()
                    },
                }
            }).fold(0, |acc, (ix, iy, direction)| {
                    let mut visited = HashSet::new();
                    traverse(&mut visited, ix, iy, direction, &map);
                    acc.max(visited.iter().map(|(ix, iy, _)| (*ix, *iy))
                        .collect::<HashSet<(usize, usize)>>().len())
                }
            );


    println!("{}", res);
}
