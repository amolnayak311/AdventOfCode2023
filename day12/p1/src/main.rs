use std::fs::read_to_string;
fn main() {

    // let chars = "???.###.".chars().collect::<Vec<char>>();
    // let arrangements = vec![1,1,3];

    // let chars = "????.######..#####.".chars().collect::<Vec<char>>();
    // let arrangements = vec![1,6,5];


    // // let chars = ".??..??...?##.".chars().collect::<Vec<char>>();
    // // let arrangements = vec![1, 1, 3];
    // //
    // // let chars = "?###????????".chars().collect::<Vec<char>>();
    // // let arrangements = vec![3,2,1];
    //
    // let chars = "?###????????.####".chars().collect::<Vec<char>>();
    // let arrangements = vec![3, 2, 1];
    //
    //
    // println!("{}", get_combinations(&chars, &arrangements, 0, 0));

    let res : u32 = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let splits = line.split(" ").collect::<Vec<&str>>();
            let chars = splits[0].chars().collect::<Vec<char>>();
            let arrangements =
                splits[1]
                    .split(",")
                    .map(|n| n.parse::<u8>().unwrap()).collect::<Vec<u8>>();
            let res = get_combinations(&chars, &arrangements, 0, 0) as u32;
            //println!("{:?}, {:?}, {}", chars, arrangements, res);
            res
        }).sum();


    println!("{}", res);

}

fn get_combinations(chars: &Vec<char>, arrangements: &Vec<u8>, c_idx: usize, a_idx: usize) -> u32 {
    if a_idx == arrangements.len() && c_idx >= chars.len() {
        1
    } else if c_idx >= chars.len() {
        0
    } else if a_idx == arrangements.len() {
        if (c_idx..chars.len()).fold(true, |acc, idx| acc && (chars[idx] == '.' || chars[idx] == '?')) {
            1
        } else {
            0
        }
    } else {
        if chars[c_idx] == '.' {
            get_combinations(chars, arrangements, c_idx + 1, a_idx)
        } else {
            // if ? replace with one . and continue
            let with_dot =
                if chars[c_idx] == '?' {
                    get_combinations(chars, arrangements, c_idx + 1, a_idx)
                } else {
                    0
                };

            // Or match the required hashes
            let expected_hashes = arrangements[a_idx];

            // take expected_hashes chars from c_idx and count the number of ? and # chars,
            // till we see a . or end of our chars
            let (num_hashes, num_questions) = (c_idx .. (c_idx + expected_hashes as usize).min(chars.len()))
                .take_while(|idx| chars[*idx] != '.')
                .fold((0, 0), |(num_hashes, num_questions), curr_idx| {
                    if chars[curr_idx] == '#' {
                        (num_hashes + 1, num_questions)
                    } else {
                        (num_hashes, num_questions + 1)
                    }
                });

            let next_idx = c_idx + num_hashes as usize + num_questions as usize;
            if num_hashes + num_questions == expected_hashes
                && (next_idx == chars.len() || chars[next_idx] == '.' || chars[next_idx] == '?') {
                with_dot + get_combinations(chars, arrangements, next_idx + 1, a_idx + 1)
            } else {
                with_dot
            }
        }
    }
}
