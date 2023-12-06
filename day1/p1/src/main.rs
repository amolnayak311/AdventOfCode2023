use std::fs::read_to_string;

fn main() {
    let radix: u32 = 10;
    let res: Vec<String>  = read_to_string("./src/input.txt").unwrap().lines().map(String::from).collect();
    let res: u32 = res.iter().map(|s|{
        let tuple = s.chars()
        .filter(|x| x.is_numeric())
        .fold((None, None),|acc, c| {
                match acc {
                    (None, None)     => (Some(c.to_digit(radix).unwrap()), Some(c.to_digit(radix).unwrap())),
                    (first @ Some(_), _) => (first, Some(c.to_digit(radix).unwrap())),
                    (None, Some(_))  => panic!()
            }
        });
        if let (Some(first), Some(second)) = tuple {
            first * 10 + second
        } else {
            0
        }
    }).sum();
    println!("{}", res);
}
