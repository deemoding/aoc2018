use std::collections::HashMap;

pub fn main() {
    let file = include_str!("input");
    let logs:Vec<&str> = file.split_terminator('\n').collect();
    let mut records:HashMap<&str, [i32; 60]> = HashMap::new();

    let mut id = "";
    let mut start:usize = 59;
    for log in logs {
        let _log:Vec<&str> = log.split_terminator(' ').collect();
        match _log[2] {
            "Guard" => {
                id = _log[3];
            }
            "falls" => {
                start = _log[1][3..5]
                    .parse::<usize>()
                    .expect("parse to u32 failed");
            }
            "wakes" => {
                let stop = _log[1][3..5]
                    .parse::<usize>()
                    .expect("parse to u32 failed");
                records.entry(&id).or_insert([0 as i32; 60]);
                for i in start..stop {
                    records.get_mut(id).unwrap()[i] += 1;
                }
            }
            _ => ()
        }
    }

    let mut max_sum = 0;
    let mut asleep_most = 0;
    let mut max_id = "";
    for (id, minute) in &records {
        let mut max = 0;
        let mut max_index = 0;
        let mut sum = 0;
        minute.iter().enumerate().for_each(|(i, m)| {
            if max < *m {
                max = *m;
                max_index = i;
            }
            sum += m;
        });
        if max_sum < sum {
            max_sum = sum;
            asleep_most = max_index;
            max_id = id;
        }
    }
    println!("part1: {}, {}", max_id, asleep_most);

    // just repeat code above
    let mut max_frequence = 0;
    let mut max_frequence_index = 0;
    let mut max_id = "";
    for (id, minute) in &records {
        let mut max = 0;
        let mut max_index = 0;
        minute.iter().enumerate().for_each(|(i, m)| {
            if max < *m {
                max = *m;
                max_index = i;
            }
        });
        if max_frequence < max {
            max_frequence = max;
            max_frequence_index = max_index;
            max_id = id;
        }
    }
    println!("part2: {}, {}", max_id, max_frequence_index);
}
