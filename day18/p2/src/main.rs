use std::fs::read_to_string;

fn main() {
    let (total_perimeter, area, _) = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let color = line.split(" ").collect::<Vec<&str>>()[2];
            let steps = i64::from_str_radix(&color[2 ..color.len() -2], 16).unwrap();
            let direction = match color[color.len() -2 ..color.len() - 1].parse::<u8>().unwrap() {
                0  => 'R',
                1  => 'D',
                2  => 'L',
                3  => 'U',
                _  => panic!("Unknown direction")
            };
            (steps, direction)
        })
        .fold((0, 0, (0, 0)),
              |(total_perimeter, area, (x, y)), (steps, direction)| {
                  match direction {
                      'L'  => (total_perimeter + steps, area, (x - steps, y)),
                      'R'  => (total_perimeter + steps, area, (x + steps, y)),
                      'D'  => (total_perimeter + steps, area + x * steps, (x, y + steps)),
                      'U'  => (total_perimeter + steps, area - x * steps, (x, y - steps)),
                      _  => panic!("Unknown direction")
                  }
        });
    println!("{}", area + total_perimeter / 2 + 1);
}
