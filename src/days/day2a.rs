use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("input/input2.txt")
        .unwrap()
        .split(",")
        .map(|id| {
            let range = id.split("-").collect::<Vec<_>>();
            (
                range[0].parse().unwrap(),
                range[1].parse().unwrap(),
            )
        })
        .collect::<Vec<(u64, u64)>>();

    // dbg!(&input);

    let mut invalid_count: u64 = 0;

    for (lower, upper) in input {
        invalid_count += add_double_ids(lower, upper);
    }

    println!("{invalid_count}");
}

fn add_double_ids(lower: u64, upper: u64) -> u64 {
    let mut invalid_sum: u64 = 0;
    for i in lower..=upper {
        let num_string = i.to_string();
        // println!("{num_string}");
        if num_string.len() % 2 == 0 {
            let first_half = num_string.split_at(num_string.len() / 2).0;
            let second_half = num_string.split_at(num_string.len() / 2).1;
            // println!("{} und {}", first_half, second_half);
            if first_half == second_half {
                invalid_sum += num_string.parse::<u64>().unwrap();
            }
        }
    }
    return invalid_sum;
}
