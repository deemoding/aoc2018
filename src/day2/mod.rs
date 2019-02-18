use std::collections::HashMap;

pub fn main() {
    let file = include_str!("input");
    let ids:Vec<&str> = file.split_terminator('\n').collect();

    let mut twice = 0;
    let mut three_times = 0;
    for id in ids.iter() {
        let mut count_map:HashMap<char, u32> = HashMap::new();
        for chr in id.chars() {
            let count = count_map.entry(chr).or_insert(0);
            *count += 1;
        }

        let mut has_two = 0;
        let mut has_three = 0;
        for value in count_map.values() {
            if *value == 3 {
                has_three = 1;
            }
            if *value == 2 {
                has_two = 1;
            }
            if has_two + has_three == 2 {
                break;
            }
        }
        twice += has_two;
        three_times += has_three;
    }
    println!("part1: {}", twice * three_times);

    let mut found = false;
    for i in 0..ids.len() - 1 {
        for j in i + 1..ids.len() {
            let mut diff = 0;
            for (a, b) in ids[i].chars().zip(ids[j].chars()) {
                if a != b {
                    diff += 1;
                }
                if diff > 1 {
                    break;
                }
            }
            if diff == 1 {
                println!("part2: {}, {}", ids[i], ids[j]);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
}
