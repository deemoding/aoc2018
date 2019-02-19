use std::collections::HashMap;

pub fn main() {
    let file = include_str!("input");
    let claims:Vec<&str> = file.split_terminator('\n').collect();
    let mut fabric = [[0 as u32; 1000]; 1000];
    let mut belongs_to = vec![vec![""; 1000]; 1000];
    let mut overlap:HashMap<&str, bool> = HashMap::new();

    claims.iter().for_each(|chaim| {
        let claim_arr:Vec<&str> = chaim.split_terminator(' ').collect();
        let xy:Vec<usize> = claim_arr[2][0..claim_arr[2].len() - 1]
            .split(',')
            .map(|n| n.parse::<usize>().expect("trans to usize failed"))
            .collect();
        let wh:Vec<usize> = claim_arr[3]
            .split('x')
            .map(|n| n.parse::<usize>().expect("trans to usize failed"))
            .collect();
        for i in xy[0]..xy[0] + wh[0] {
            for j in xy[1]..xy[1] + wh[1] {
                fabric[i][j] += 1;
                if belongs_to[i][j] == "" {
                    belongs_to[i][j] = claim_arr[0];
                    // 重点：不能直接覆盖原有值，否则在部分重复的地方会出错
                    overlap.entry(claim_arr[0]).or_insert(false);
                } else {
                    overlap.insert(belongs_to[i][j], true);
                    overlap.insert(claim_arr[0], true);
                }
            }
        }
    });

    let mut areas = 0;
    for i in 0..1000 as usize {
        for j in 0..1000 as usize {
            if fabric[i][j] > 1 {
                areas += 1;
            }
        }
    }
    println!("part1: {}", areas);

    println!("part2:");
    for (id, is_overlap) in overlap {
        if !is_overlap {
            println!("{}", id);
        }
    }
}
