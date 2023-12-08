use std::fs::read_to_string;

fn build_mapping_functions() -> (Vec<u64>, fn(u64) -> u64, fn(u64) -> u64, fn(u64) -> u64, fn(u64) -> u64, fn(u64) -> u64, fn(u64) -> u64, fn(u64) -> u64) {

    let seeds = "280775197 7535297 3229061264 27275209 77896732 178275214
                 2748861189 424413807 3663093536 130341162 613340959 352550713 
                 1532286286 1115055792 1075412586 241030710 3430371306 138606714 412141395 146351614"
                 .replace("\n", "").split(" ").filter(|x|x.trim() != "").map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    // let seeds = "79 14 55 13"
    //               .replace("\n", "").split(" ").filter(|x|x.trim() != "").map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    
    

    let seed_to_soil = |x| {
        let file_raw = read_to_string("./src/input.txt").unwrap();
        let mut start = false;
        let mut raw_string_lines = vec![];
        for line in file_raw.lines() {
            if line == "seed-to-soil map:" {
                start = true
            } else if start && line.trim() == "" {
                break;
            } else if start {
                raw_string_lines.push(line);
            }
        }
        
        let filtered = raw_string_lines.iter()
        .map(|split| {
            let split_tuple = split.trim().split(' ').map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            (split_tuple[0], split_tuple[1], split_tuple[2])
        }).filter(|tuple| x >= tuple.1 && x <= tuple.1 + tuple.2).collect::<Vec<(u64, u64, u64)>>();
        
        if filtered.len() > 0 {
            filtered[0].0 + (x - filtered[0].1)
        } else {
            x
        }

    };

    let soil_to_fertilizer = |x| {
        let file_raw = read_to_string("./src/input.txt").unwrap();
        let mut start = false;
        let mut raw_string_lines = vec![];
        for line in file_raw.lines() {
            if line == "soil-to-fertilizer map:" {
                start = true
            } else if start && line.trim() == "" {
                break;
            } else if start {
                raw_string_lines.push(line);
            }
        }
        //println!("{:?}", raw_string_lines);
        let filtered = raw_string_lines.iter()
        .map(|split| {
            let split_tuple = split.trim().split(' ').map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            (split_tuple[0], split_tuple[1], split_tuple[2])
        }).filter(|tuple| x >= tuple.1 && x <= tuple.1 + tuple.2).collect::<Vec<(u64, u64, u64)>>();
        //println!("{:?}", filtered);
        if filtered.len() > 0 {
            filtered[0].0 + (x - filtered[0].1)
        } else {
            x
        }

    };

    let fertilizer_to_water = |x| {
        let file_raw = read_to_string("./src/input.txt").unwrap();
        let mut start = false;
        let mut raw_string_lines = vec![];
        for line in file_raw.lines() {
            if line == "fertilizer-to-water map:" {
                start = true
            } else if start && line.trim() == "" {
                break;
            } else if start {
                raw_string_lines.push(line);
            }
        }
        
        let filtered = raw_string_lines.iter()
        .map(|split| {
            let split_tuple = split.trim().split(' ').map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            (split_tuple[0], split_tuple[1], split_tuple[2])
        }).filter(|tuple| x >= tuple.1 && x <= tuple.1 + tuple.2).collect::<Vec<(u64, u64, u64)>>();
        
        if filtered.len() > 0 {
            filtered[0].0 + (x - filtered[0].1)
        } else {
            x
        }

    };


    let water_to_light = |x| {
        let file_raw = read_to_string("./src/input.txt").unwrap();
        let mut start = false;
        let mut raw_string_lines = vec![];
        for line in file_raw.lines() {
            if line == "water-to-light map:" {
                start = true
            } else if start && line.trim() == "" {
                break;
            } else if start {
                raw_string_lines.push(line);
            }
        }
        
        let filtered = raw_string_lines.iter()
        .map(|split| {
            let split_tuple = split.trim().split(' ').map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            (split_tuple[0], split_tuple[1], split_tuple[2])
        }).filter(|tuple| x >= tuple.1 && x <= tuple.1 + tuple.2).collect::<Vec<(u64, u64, u64)>>();
        
        if filtered.len() > 0 {
            filtered[0].0 + (x - filtered[0].1)
        } else {
            x
        }

    };


    let light_to_temperature = |x| {
        let file_raw = read_to_string("./src/input.txt").unwrap();
        let mut start = false;
        let mut raw_string_lines = vec![];
        for line in file_raw.lines() {
            if line == "light-to-temperature map:" {
                start = true
            } else if start && line.trim() == "" {
                break;
            } else if start {
                raw_string_lines.push(line);
            }
        }
        
        let filtered = raw_string_lines.iter()
        .map(|split| {
            let split_tuple = split.trim().split(' ').map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            (split_tuple[0], split_tuple[1], split_tuple[2])
        }).filter(|tuple| x >= tuple.1 && x <= tuple.1 + tuple.2).collect::<Vec<(u64, u64, u64)>>();
        
        if filtered.len() > 0 {
            filtered[0].0 + (x - filtered[0].1)
        } else {
            x
        }

    };


    let temperature_to_humidity = |x| {
        let file_raw = read_to_string("./src/input.txt").unwrap();
        let mut start = false;
        let mut raw_string_lines = vec![];
        for line in file_raw.lines() {
            if line == "temperature-to-humidity map:" {
                start = true
            } else if start && line.trim() == "" {
                break;
            } else if start {
                raw_string_lines.push(line);
            }
        }
        
        let filtered = raw_string_lines.iter()
        .map(|split| {
            let split_tuple = split.trim().split(' ').map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            (split_tuple[0], split_tuple[1], split_tuple[2])
        }).filter(|tuple| x >= tuple.1 && x <= tuple.1 + tuple.2).collect::<Vec<(u64, u64, u64)>>();
        
        if filtered.len() > 0 {
            filtered[0].0 + (x - filtered[0].1)
        } else {
            x
        }

    };


    let humidity_to_location = |x| {
        let file_raw = read_to_string("./src/input.txt").unwrap();
        let mut start = false;
        let mut raw_string_lines = vec![];
        for line in file_raw.lines() {
            if line == "humidity-to-location map:" {
                start = true
            } else if start && line.trim() == "" {
                break;
            } else if start {
                raw_string_lines.push(line);
            }
        }
        
        let filtered = raw_string_lines.iter()
        .map(|split| {
            let split_tuple = split.trim().split(' ').map(|x|x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
            (split_tuple[0], split_tuple[1], split_tuple[2])
        }).filter(|tuple| x >= tuple.1 && x <= tuple.1 + tuple.2).collect::<Vec<(u64, u64, u64)>>();
        
        if filtered.len() > 0 {
            filtered[0].0 + (x - filtered[0].1)
        } else {
            x
        }

    };
 
    (seeds,
    seed_to_soil,
    soil_to_fertilizer,
    fertilizer_to_water,
    water_to_light,
    light_to_temperature,
    temperature_to_humidity,
    humidity_to_location)

   
}
fn main() {
    let (seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location) = build_mapping_functions();
    let min = seeds.into_iter().map(|x| 
        humidity_to_location(
            temperature_to_humidity(
                light_to_temperature(
                    water_to_light(
                        fertilizer_to_water(
                            soil_to_fertilizer(
                                seed_to_soil(x)
                            )
                        )
                    )
                )
            )
        )
    ).fold(4294967295, |acc, x| acc.min(x));

    // println!("{}", 
    // humidity_to_location(
    //     temperature_to_humidity(
    //         light_to_temperature(
    //             water_to_light(
    //                 fertilizer_to_water(
    //                     soil_to_fertilizer(
    //                         seed_to_soil(14)
    //                     )
    //                 )
    //             )
    //         )
    //     ))
    // );
    // println!("{}", soil_to_fertilizer(14));
    println!("{}", min);
}
