use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("error reading from file");

    let rows: Vec<&str> = input.lines().collect();
    let mut numbers: Vec<Vec<&str>> = Vec::with_capacity(rows.len());
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(rows.len());
    let mut flipped_numbers: Vec<Vec<String>> = Vec::with_capacity(rows.len());

    let row_len = rows.len();
    let row_count = rows[0].split_whitespace().count();
    let row_count_char = rows[0].len();

    let mut operations: Vec<&str> = vec![" "; row_count];
    let mut results: Vec<i128> = vec![0; row_count];

    for i in 0..row_len {
        numbers.push(vec![" "; row_count]);
        flipped_numbers.push(vec![String::from(""); row_count]);
        if i != row_len - 1 {
            grid.push(vec![' '; row_count_char]);
        }
    }

    for (i, row) in rows.iter().enumerate() {
        for (j, num) in row.split_whitespace().enumerate() {
            if i == row_len - 1 {
                operations[j] = num;
                continue;
            }
            numbers[i][j] = num;
        }
    }

    for (i, row) in rows.iter().enumerate() {
        for (j, char) in row.chars().enumerate() {
            if i == row_len - 1 {
                continue;
            }
            grid[i][j] = char;
        }
    }

    let mut column_counter = 0;
    let mut len = 0;
    for j in 0..row_count_char {
        let mut is_empty_column = true;
        for i in 0..row_len - 1 {
            if grid[i][j] != ' ' {
                is_empty_column = false;
            }
            println!("char - {} - {} {} {}", grid[i][j], is_empty_column, i, j);
        }
        println!("len {} {}", len, column_counter);
        if !is_empty_column{
            len += 1;
        } else {
            for i in 0..row_len - 1 {
                for k in 0..len {
                    if grid[i][j - k - 1] != ' ' {
                        println!("upisuje {} u {}", grid[i][j-k-1], flipped_numbers[k][column_counter]);
                        flipped_numbers[k][column_counter].push(grid[i][j - k-1]);
                    }
                }
            }
            len = 0;
            column_counter += 1;
        }
        println!("len2 {}", len);
    }

    for j in 0..row_count {
        let mut column_total: i128 = 0;
        let is_multiply = operations[j] == "*";
        if is_multiply {
            column_total = 1;
        }

        for i in 0..row_len - 1 {
            if flipped_numbers[i][j].len() == 0 {continue;}
            
            if is_multiply {
                column_total *= flipped_numbers[i][j]
                    .parse::<i128>()
                    .expect("error in parse");
            } else {
                column_total += flipped_numbers[i][j]
                    .parse::<i128>()
                    .expect("error in parse");
            }
        }

        results[j] = column_total;
    }

    let mut total: i128 = 0;

    for num in results {
        total += num;
    }

    println!("total - {}", total)
}
