use std::fs::read_to_string;

pub fn main() {
    let mut counter = 0;
    let mut dial = 50;
    read_to_string("input/input1.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let x = l.replace("L", "-").replace("R", "").parse::<i32>().unwrap();
            (x.abs() / 100, x % 100)
        })
        .for_each(|(fulls, partial)| {
            counter += fulls;
            if dial + partial < 0 || dial + partial > 100 {
                counter += 1;
                dial = (100 + dial + partial) % 100;
            } else if dial + partial == 0 || dial + partial == 100 {
                counter += 1;
                dial = 0;
            } else {
                dial = dial + partial;
            }
        });

    // full_rotations counts the times the dial has to cross over zero, because once per hundred, it has to
    // degree counts the rest, which when after rotating, it would overflow the dial, it gets wrapped back into by 100 and the zero gets crossed over once
    // if at the end of a turn, the dial stops on zero and was not at zero before, an additional zero is added for that
    // for (direction, full_rotations, mut degree) in instructions {
    //     counter += full_rotations;
    //     if dial

    //     if degree == 0 {
    //         continue;
    //     } else if (dial + degree) < 0 {
    //         if dial != 0 {
    //             counter += 1;
    //         }
    //         dial = dial + degree + 100;
    //     } else if (dial + degree) > 100 {
    //         dial = dial + degree - 100;
    //         counter += 1;
    //     } else if (dial + degree) == 100 || (dial + degree) == 0 {
    //         dial = 0;
    //         counter += 1;
    //     } else {
    //         dial = dial + degree;
    //     }
    //     // println!("Deg: {degree}, Post: {dial}, Fulls: {full_rotations}, Pass: {password}");
    // }
    println!("{counter}");
}
