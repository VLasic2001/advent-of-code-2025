use core::num;
use std::fs;

fn main() {
    let ranges_string = fs::read_to_string("input.txt").expect("error with read");

    let ranges = ranges_string.split(",");

    let mut vec: Vec<i64> = Vec::new();

    for range in ranges {
        let (start, end) = split_range(range);

        check_through_range(
            &mut vec,
            start.parse().expect("error with parse"),
            end.parse().expect("error with parse"),
        );
    }
    let mut total = 0;

    for num in vec {
        total += num;
    }

    println!("{total}");
}

fn split_range(range: &str) -> (&str, &str) {
    let bytes = range.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'-' {
            return (&range[..i], &range[i + 1..]);
        }
    }

    (&range[..], &range[..])
}

fn check_through_range(vec: &mut Vec<i64>, start: i64, end: i64) {
    for id in start..end + 1 {
        let id_str = id.to_string();
        let mut denoms: Vec<usize> = Vec::new();
        for num in 1..id_str.len() {
            if (id_str.len() as f64 / num as f64).fract() == 0.0 {
                denoms.push(num as usize);
            }
        }

        for denom in denoms {
            let num_to_check = &id_str[..denom];
            let mut i = 0;
            let mut is_valid = true;
            loop {
                if &id_str[i * denom..(i + 1) * (denom)] != num_to_check {
                    is_valid = false;
                    break;
                };

                if (i + 2) * denom > id_str.len() {
                    break;
                }

                i += 1;
            }

            if is_valid == true && !vec.contains(&id) {
                vec.push(id);
            }
        }
    }
}
