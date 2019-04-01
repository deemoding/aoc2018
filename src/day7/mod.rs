pub fn main() {
    let file:&str = include_str!("input");
    let input:Vec<[usize; 2]> = file
        .split_terminator('\n')
        .map(|x:&str| {
            let tmp = x.as_bytes();
            return [(tmp[5] - 65) as usize, (tmp[36] - 65) as usize];
        })
        .collect();

    let mut parents:[u32; 26] = [0; 26];
    let mut children:[u32; 26] = [0; 26];
    input.iter().for_each(|x| {
        parents[x[1]] |= 1 << x[0];
        children[x[0]] |= 1 << x[1];
    });
    // 找到所有一级任务
    let mut queue: u32 = 0;
    for ascii in 0..26 as usize {
        if parents[ascii] == 0 && children[ascii] > 0 {
            queue |= 1 << ascii;
        }
    }

    let mut part1:Vec<usize> = vec![];
    // 记录已完成任务
    let mut done:u32 = 0;
    while queue > 0 {
        let mut ascii:usize = 0;
        while queue & (1 << ascii) == 0 {
            ascii += 1;
        }
        part1.push(ascii);
        done |= 1 << ascii;
        // 查找子代的父亲是否全部完成，只有全部完成的子代才能加入队列
        for child in 0..26 as usize {
            if (children[ascii] & (1 << child)) > 0 && parents[child] & done == parents[child] {
                queue |= 1 << child;
            }
        }
        // 移除所有已完成任务
        queue &= !done;
    }
    print!("part1: ");
    part1.iter().for_each(|chr| {
        print!("{}", char::from((chr + 65) as u8));
    });
    println!();
}
