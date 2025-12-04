use std::fs;

fn main() {
    let rows_string = fs::read_to_string("input.txt").expect("error with read");

    let mut rows: Vec<Vec<char>> = rows_string
        .split_whitespace()
        .map(|r| r.chars().collect())
        .collect();

    let row_len = rows.len();
    let row_count = rows[0].len();

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for _ in 0..row_count {
        grid.push(vec![0; row_len]);
    }

    calculate_grid(&mut grid, &mut rows, row_count, row_len);

    let mut total_hit_count = 0;

    loop {
        let mut current_hit_count = 0;

        reset_grid(&mut grid, row_count, row_len);
        calculate_grid(&mut grid, &mut rows, row_count, row_len);

        for (i, row) in rows.iter_mut().enumerate() {
            for (j, char) in row.iter_mut().enumerate() {
                if *char == '.' {
                    *char = '.';
                } else if *char == '@' && grid[i][j] < 4 {
                    total_hit_count += 1;
                    current_hit_count += 1;
                    *char = '.';
                } else {
                    *char = '@';
                }
                print!("{char}");
            }
            println!();
        }

        println!();
        println!("{} {}", current_hit_count, total_hit_count);
        println!();
        if current_hit_count == 0 {
            break;
        }
    }

    println!("hit {total_hit_count}");
}

fn reset_grid(grid: &mut Vec<Vec<i32>>, row_count: usize, row_len: usize) {
    for i in 0..row_count {
        for j in 0..row_len {
            grid[i][j] = 0;
        }
    }
}

fn calculate_grid(
    grid: &mut Vec<Vec<i32>>,
    rows: &mut Vec<Vec<char>>,
    row_count: usize,
    row_len: usize,
) {
    for (i, row) in rows.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == '@' {
                if i >= 1 && j >= 1 {
                    grid[i - 1][j - 1] += 1;
                }
                if i >= 1 {
                    grid[i - 1][j] += 1;
                }
                if i >= 1 && j < row_len - 1 {
                    grid[i - 1][j + 1] += 1;
                }

                if j >= 1 {
                    grid[i][j - 1] += 1;
                }
                if j < row_len - 1 {
                    grid[i][j + 1] += 1;
                }

                if i < row_count - 1 && j >= 1 {
                    grid[i + 1][j - 1] += 1;
                }
                if i < row_count - 1 {
                    grid[i + 1][j] += 1;
                }
                if i < row_count - 1 && j < row_len - 1 {
                    grid[i + 1][j + 1] += 1;
                }
            }
        }
    }
}
