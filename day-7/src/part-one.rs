use std::fs;

fn main() {
    let mut grid: Vec<Vec<char>> = fs::read_to_string("input.txt").unwrap().lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut counter = 0;

    for i in 0..grid.len()-1 {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                grid[i + 1][j] = '|';
            } else if grid[i][j] == '^' && grid[i - 1][j] == '|' {
                counter += 1;
                grid[i][j - 1] = '|';
                grid[i + 1][j - 1] = '|';
                grid[i][j + 1] = '|';
            } else if grid[i][j] == '|' && grid[i + 1][j] == '.'{
                grid[i + 1][j] = '|';
            }
        }
    }

    for row in grid {
        for char in row {
            print!("{}", char);
        }
        println!();
    }

    println!("{}", counter);
}
