fn manhattan_distance(x:&[i32; 2], y:&[i32; 2]) -> i32 {
    return (x[0] - y[0]).abs() + (x[1] - y[1]).abs();
}

// 查找每个点距离谁最近，min_distance=-1为非法值
fn find_nearest(x: i32, y: i32, coords:&Vec<Vec<i32>>) -> (i32, usize) {
    let mut min_distance:i32 = 9999;
    let mut min_index:usize = 0;
    coords.iter().enumerate().for_each(|(index, coord)| {
        let distance = manhattan_distance(&[x, y], &[coord[0], coord[1]]);
        // 如果和多个点距离相同，视为无效
        if min_distance == distance {
            min_distance = -1;
        }
        if min_distance > distance {
            min_distance = distance;
            min_index = index;
        }
    });
    return (min_distance, min_index);
}

pub fn main() {
    let file:&str = include_str!("input");
    let mut xmin:i32 = 9999;
    let mut ymin:i32 = 9999;
    let mut xmax:i32 = 0;
    let mut ymax:i32 = 0;
    let mut areas: Vec<u32> = Vec::new();
    let mut coords:Vec<Vec<i32>> = file
        .split_terminator('\n')
        .map(|coord:&str| {
            let xy:Vec<i32> = coord
                .split_terminator(", ")
                .map(|xy:&str| xy.parse::<i32>().expect("xy trans to i32 failed"))
                .collect();
            if xmin > xy[0] {
                xmin = xy[0];
            }
            if ymin > xy[1] {
                ymin = xy[1];
            }
            if xmax < xy[0] {
                xmax = xy[0];
            }
            if ymax < xy[1] {
                ymax = xy[1];
            }
            areas.push(0);
            return xy;
        })
        .collect();
    coords = coords.iter()
        .map(|coord:&Vec<i32>| {
            return vec![coord[0] - xmin, coord[1] - ymin];
        })
        .collect();
    xmax -= xmin;
    ymax -= ymin;

    let mut boundary:u64 = 0;
    (0..xmax + 1).for_each(|t| {
        let (min_distance, min_index) = find_nearest(t, 0, &coords);
        if min_distance != -1 {
            boundary |= 1 << min_index;
        }
        let (min_distance, min_index) = find_nearest(t, ymax, &coords);
        if min_distance != -1 {
            boundary |= 1 << min_index;
        }
    });
    (0..ymax + 1).for_each(|t| {
        let (min_distance, min_index) = find_nearest(0, t, &coords);
        if min_distance != -1 {
            boundary |= 1 << min_index;
        }
        let (min_distance, min_index) = find_nearest(xmax, t, &coords);
        if min_distance != -1 {
            boundary |= 1 << min_index;
        }
    });

    let mut max_area:u32 = 0;
    for i in 1..xmax {
        for j in 1..ymax {
            let (min_distance, min_index) = find_nearest(i, j, &coords);
            if min_distance == -1 {
                continue;
            }
            if (boundary & (1 << min_index)) != 0 {
                continue;
            }
            areas[min_index] += 1;
            if max_area < areas[min_index] {
                max_area = areas[min_index]
            }
        }
    }

    println!("part1: {}", max_area);
}
