pub fn main() {
    let file = include_str!("input");
    let mut s = String::from(file);
    let mut point:usize = 0;
    while point < s.len() - 1 {
        if (&s[point..point + 1].to_lowercase() == &s[point + 1..point + 2].to_lowercase())
            && (&s[point..point + 1] != &s[point + 1..point + 2]) {
            s.replace_range(point..point + 2, "");
            // point -= 1;
            point = point.saturating_sub(1);
        } else {
            point += 1;
        }
    }
    // 忽略结尾的换行符号
    println!("part1: {}", s.len() - 1);
}
