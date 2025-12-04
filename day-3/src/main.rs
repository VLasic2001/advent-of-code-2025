use std::fs;

const NUM_OF_BATTERIES: usize = 12;

fn main() {
    let banks_string = fs::read_to_string("input.txt").expect("error with read");

    let banks = banks_string.split_whitespace();

    let mut results: Vec<String> = Vec::new();

    for bank in banks {
        results.push(find_positions(bank));
    }

    let mut total = 0;

    for res in results {
        println!("{res}");
        total += res.parse::<i64>().expect("error with parse");
    }

    println!("{total}");
}

fn find_positions(bank: &str) -> String {
    let mut res = [0; NUM_OF_BATTERIES];
    let mut res_index = [0; NUM_OF_BATTERIES];
    let mut skips = bank.len() - NUM_OF_BATTERIES;
    
    for i in 0..NUM_OF_BATTERIES {
        let mut start_index = 0;
        if i > 0 {
            start_index = res_index[i - 1] + 1;
        }
        let str = &bank[(start_index)..(skips + start_index+ 1)];
        for (j, char) in str.chars().enumerate() {
            let num = char as i64 - 0x30;
            if num > res[i] {
                res[i] = num;
                res_index[i] = start_index + j;
            }
        }
        skips -= res_index[i] - start_index;
    }

    let mut result_str = res[0].to_string();
    for i in 1..NUM_OF_BATTERIES {
        result_str.push_str(&res[i].to_string());
    }
    String::from(result_str)
}
