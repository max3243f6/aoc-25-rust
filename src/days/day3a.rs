use std::fs::read_to_string;

pub fn main() {
    let sum = read_to_string("input/sample3.txt").unwrap().lines().map(|line| {
        10 * line.chars()
    }).fold(0, |acc, x| acc + x);
    println!("{sum}");
}