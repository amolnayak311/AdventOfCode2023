use std::fs::read_to_string;
fn hash(str: &str) -> u8 {
    str.chars().fold(0, |acc, chr| {
        let temp_acc = 17 * (acc as u32 + chr as u32);
        (temp_acc % 256) as u8
    })
}
fn main() {

    let unwrapped_file = read_to_string("./src/input.txt").unwrap();
    let res = unwrapped_file
        .lines()
        .fold(vec![vec![]; 256],
              |mut acc, x| {
                  x.split(",")
                      .for_each(|command| {
                          if command.contains("=") {
                              let idx = command.find("=").unwrap();
                              let box_label = &command[0..idx];
                              let focal_length = &command[idx + 1..].parse::<u32>().unwrap();
                              let lhm = &mut acc[hash(box_label) as usize];
                              match lhm.iter().position(| x: &(&str, u32)| x.0 == box_label)  {
                                  Some(idx)   =>   lhm[idx] = (box_label, *focal_length),
                                  None        =>   lhm.push((box_label, *focal_length))
                              }
                          } else {
                              let box_label = &command[0.. command.len() - 1];
                              let lhm = &mut acc[hash(box_label) as usize];
                              if let Some(idx) = lhm.iter().position(| x: &(&str, u32)| x.0 == box_label) {
                                  lhm.remove(idx);
                              }
                          }
                      });
                    acc
              }
        ).iter().zip(1..257).fold(0, |acc, (vec, box_idx)| {
                acc + box_idx * vec.iter().zip(1..vec.len() + 1).fold(0,
                    |box_score, (item, item_idx)| {
                        box_score + item.1 * item_idx as u32
                })
            });
    println!("{:?}", res);
}
