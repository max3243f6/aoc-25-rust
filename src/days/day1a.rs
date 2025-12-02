use std::fs::read_to_string;

pub fn main() {
    let instructions = read_to_string("input/input1.txt").unwrap().lines().map(|l| {
        (
            l.split_at(1).0.parse::<char>().unwrap(),
            l.split_at(1).1.parse::<i32>().unwrap()%100
        )
    }).collect::<Vec<_>>();

    let mut password = 0;
    let mut dial = 50;

    for (direction, mut degree) in instructions {
        if direction == 'L' {
            degree *= -1;
        }
        if (dial + degree) < 0 {
            dial = dial + degree + 100;
        } else if (dial + degree) > 99 {
            dial = dial + degree - 100;
        } else {
            dial = dial + degree;
        }
        if dial == 0 {
            password += 1;
        }
    }
    println!("{password}")
}
