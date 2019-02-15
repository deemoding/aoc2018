pub fn main() {
    let file = include_str!("input");
    let nums:Vec<&str> = file.split_terminator('\n').collect();
    let sum:i32 = nums.iter()
        .map(|x| x.parse::<i32>().expect("trans to i32 error"))
        .sum();
    println!("{}", sum);
}
