use std::fs::read_to_string;
fn find_mirrors(pattern: &Vec<&str>) -> u32 {

    // Check for horizontal pattern
    for r_idx in 1..pattern.len() {
        let max_span = r_idx.min(pattern.len() - r_idx);
        if (r_idx - max_span..r_idx).zip((r_idx .. r_idx + max_span).rev())
            .fold(0, |acc, (left_idx, right_idx)|
                // Check the difference between these to be exactly one char
                acc +
                    (pattern[left_idx].bytes().zip(pattern[right_idx].bytes()).fold(0,
                            |acc, (l_char, r_char)|
                                if l_char != r_char {acc + 1} else {acc}))
            ) == 1 {
            return 100 * r_idx as u32;
        }
    }

    let nc = pattern[0].len();
    // Check for vertical pattern
    for c_idx in 1..nc {
        let max_span = c_idx.min(nc - c_idx);
        if (c_idx - max_span..c_idx).zip((c_idx .. c_idx + max_span).rev())
            .fold(0, |acc, (left_idx, right_idx)| {
                // Check the difference between these to be no more than 1
                acc + ((0..pattern.len()).fold(0, |inner_acc, r_idx| {
                    let bytes = pattern[r_idx].as_bytes();
                    if bytes[left_idx] != bytes[right_idx] {
                        inner_acc + 1
                    } else {
                        inner_acc
                    }
                }
                ))
            }
            ) == 1 {
            return c_idx as u32;
        }
    }
    0
}
fn main() {
    let un_wrapped_file = read_to_string("./src/input.txt").unwrap();
    let (mut patterns, final_pattern) =
        un_wrapped_file.lines()
            .fold((vec![], vec![]), |(mut acc, mut curr_pattern), line| {
                if line == "" {
                    acc.push(curr_pattern);
                    (acc, vec![])
                } else {
                    curr_pattern.push(line);
                    (acc, curr_pattern)
                }
            });
    patterns.push(final_pattern);

    println!("{}", patterns.iter().map(|p| find_mirrors(p)).sum::<u32>());

}
