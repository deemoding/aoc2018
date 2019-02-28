use regex::Regex;

fn react(polymer: &String) -> usize {
    let mut point:usize = 0;
    let mut s = String::clone(polymer);
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
    return s.len() - 1;
}

pub fn main() {
    let file = include_str!("input");
    let s = String::from(file);
    println!("part1: {}", react(&s));

    let mut min:usize = 99999;
    for i in b'a'..=b'z' {
        let reg = format!("(?i){}*", String::from_utf8(vec![i]).unwrap());
        let re = Regex::new(&reg).unwrap();
        let length = react(&re.replace_all(&s, "").to_string());
        /* 解法2
    for i in (b'a'..=b'z').map(|c| (c as char)) {
        let mut tmp = String::clone(&s);
        tmp.retain(|c| c != i && c.to_string() != i.to_uppercase().to_string());
        let length = react(&tmp);
        */
        if length < min {
            min = length;
        }
    }
    println!("part2: {}", min);
}
