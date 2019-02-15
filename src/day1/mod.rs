use std::collections::HashMap;

pub fn main() {
    let file = include_str!("input");
    let nums:Vec<i32> = file
        .split_terminator('\n')
        .map(|x: &str| x.parse::<i32>().expect("trans to i32 error"))
        .collect();
    let sum:i32 = nums.iter().sum();
    println!("part1: {}", sum);

    let mut reach_twice:HashMap<i32, bool> = HashMap::new();
    let mut sum:i32 = 0;
    let mut i = 0;
    let len = nums.len();
    loop {
        sum += &nums[i];
        if reach_twice.get(&sum).is_some() {
            println!("part2: {}", sum);
            break;
        } else {
            reach_twice.insert(sum, true);
            i += 1;
            if i >= len {
                i = 0;
            }
        }
    }
}
