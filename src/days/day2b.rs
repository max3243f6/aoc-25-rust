use std::fs::read_to_string;

pub fn main() {
    let input = read_to_string("input/input2.txt")
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
            // Check if i's length is cleanly divisible by j
            if i.to_string().len() % j == 0 {
                // Calculate the rest of i when trying to divide by the decimal place of j
                let subnumber: u64 = i % (10_u64.pow(j as u32));

                // Create test number to be checked against i
                let mut test_number: u64 = 0;

                // Calculate test number from repeated multiples of j-decimal and subnumber and compare to i
                for k in 0..=(i.to_string().len() / j - 1) {
                    test_number += 10_u64.pow((k * j) as u32) * subnumber;
                }

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
