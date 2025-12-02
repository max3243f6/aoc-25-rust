use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("input/sample2.txt")
        .unwrap()
        .split(",")
        .map(|id| {
            let range = id.split("-").collect::<Vec<_>>();
            (range[0].parse().unwrap(), range[1].parse().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    // dbg!(&input);

    let mut invalid_count: u64 = 0;

    for (lower, upper) in input {
        invalid_count += add_multiple_ids(lower, upper);
    }

    println!("{invalid_count}");
}

fn add_multiple_ids(lower: u64, upper: u64) -> u64 {
    let mut invalid_sum: u64 = 0;
    // Look at each number in the given range
    for i in lower..=upper {
        // First, set number to valid
        let mut valid = true;
        // Go through all divisors up to half of the digit length and that do not fraction i
        for j in 1..=(i.to_string().len() / 2) {
            if i.to_string().len() % j == 0 {
                // Calculate the rest of i when trying to divide by j
                let subnumber: u64 = i % (10 * j as u64);
                let mut test_number: u64 = 0;
                // Create test number of repeated subnumbers and compare to i
                for k in 1..=i.to_string().len() / j {
                    test_number += 10_u64.pow((k * j) as u32 - 1) * subnumber;
                }
                println!("{test_number} vs {i}");
                if test_number == i {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            invalid_sum += i;
        }
    }
    return invalid_sum;
}
