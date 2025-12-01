use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("input/input1.txt").unwrap();

    let line_vec = input.lines().collect::<Vec<&str>>();

    let tuple_vec: Vec<(char, usize)> = Vec::new();

    for line in line_vec {
        let (mut first, mut second) = line.split_at(1);
        first = first.parse::<char>().unwrap();
        second.parse::<usize>();
        tuple_vec.push((first, second));
    }
}
