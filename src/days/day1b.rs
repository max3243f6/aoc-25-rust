use std::fs::read_to_string;

pub fn main() {
    let instructions = read_to_string("input/input1.txt")
        .unwrap()
        .lines()
        .map(|l| {
            (
                l.split_at(1).0.parse::<char>().unwrap(),
                l.split_at(1).1.parse::<i32>().unwrap() / 100,
                l.split_at(1).1.parse::<i32>().unwrap() % 100,
            )
        })
        .collect::<Vec<_>>();

    let mut password = 0;
    let mut dial = 50;

    // full_rotations counts the times the dial has to cross over zero, because once per hundred, it has to
    // degree counts the rest, which when after rotating, it would overflow the dial, it gets wrapped back into by 100 and the zero gets crossed over once
    // if at the end of a turn, the dial stops on zero and was not at zero before, an additional zero is added for that
    for (direction, full_rotations, mut degree) in instructions {
        print!("Pre: {dial}, ");
        if direction == 'L' {
            degree *= -1;
        }
        password += full_rotations;
        if degree == 0 {
            continue;
        } else if (dial + degree) < 0 {
            if dial != 0 {
                password += 1;
            }
            dial = dial + degree + 100;
        } else if (dial + degree) > 100 {
            dial = dial + degree - 100;
            password += 1;
        } else if (dial + degree) == 100 || (dial + degree) == 0 {
            dial = 0;
            password += 1;
        } else {
            dial = dial + degree;
        }
        println!("Deg: {degree}, Post: {dial}, Fulls: {full_rotations}, Pass: {password}");
    }
    println!("{password}");
}
