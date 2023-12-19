fn get_combinations(chars: &Vec<char>, arrangements: &Vec<u8>, c_idx: usize, a_idx: usize) -> u32 {
    if a_idx >= arrangements.len() && c_idx == chars.len() {
        1
    } else if a_idx == arrangements.len() {
        // Ensure no more ? are remaining
        if (c_idx..chars.len()).fold(true, |acc, c| acc && chars[c] != '?') {
            1
        } else {
            0
        }
    } else if c_idx == chars.len() {
        // We are left with arrangements but no more chars to replace
        0
    } else {
        if chars[c_idx] == '.' {
            get_combinations(chars, arrangements, c_idx + 1, a_idx)
        } else {
            // Two cases, replace ? with a .
            let with_dot = get_combinations(chars, arrangements, c_idx + 1, a_idx);

            let expected_hashes = arrangements[a_idx];

            // take expected_hashes chars from c_idx and count the number of ? and # chars,
            // till we see a . or end of our string

            let (num_hashes, num_questions) = (c_idx .. (c_idx + expected_hashes as usize).min(chars.len() - 1))
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
                && (chars[next_idx] == '.' || chars[next_idx] == '?') {
                with_dot + get_combinations(chars, arrangements, next_idx + 1, a_idx + 1)
            } else {
                with_dot
            }
        }
    }



}

fn main() {

    // let chars = "???.###.".chars().collect::<Vec<char>>();
    // let arrangements = vec![1,1,3];

    //let chars = "?###????????.".chars().collect::<Vec<char>>();
    //let arrangements = vec![3,2,1];

    let chars = "????.######..#####.".chars().collect::<Vec<char>>();

    println!("{}", get_combinations(&chars, &arrangements, 0, 0));
}
