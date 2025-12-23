use std::{collections::{HashMap, VecDeque}, fs};

#[derive(Debug)]
struct Entry {
    i: usize,
    j: usize,
}

fn main() {
    let mut grid: Vec<Vec<char>> = fs::read_to_string("input.txt").unwrap().lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();

    let row_count = grid.len();
    let mut counter = 0;
    let mut start: Entry = Entry { i: 0, j: 0 };

    for i in 0..grid.len()-1 {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                grid[i + 1][j] = '|';
                start = Entry { i: i + 1, j: j };
            } else if grid[i][j] == '^' && grid[i - 1][j] == '|' {
                grid[i][j - 1] = '|';
                grid[i + 1][j - 1] = '|';
                grid[i][j + 1] = '|';
            } else if grid[i][j] == '|' && grid[i + 1][j] == '.'{
                grid[i + 1][j] = '|';
            }
        }
    }

    for row in &grid {
        for char in row {
            print!("{}", char);
        }
        println!();
    }

    let mut counters: HashMap<String, u128> = HashMap::new();
    counters.insert(format_entry(&start), 1);

     for i in 1..grid.len()-1 {
        for j in 0..grid[i].len() {
            if grid[i][j] == '|' && grid[i + 1][j] == '|' {
                let current = counters.get(&format_point(i, j)).unwrap();
                *counters.entry(format_point(i + 1, j)).or_insert(0) += *current;
            } else if grid[i][j] == '|' && grid[i + 1][j] == '^'{
                let current = counters.get(&format_point(i, j)).unwrap().clone();
                *counters.entry(format_point(i + 1, j - 1)).or_insert(0) += current;
                *counters.entry(format_point(i + 1, j + 1)).or_insert(0) += current;
            }
        }
    }

    for j in 0..grid[0].len() {
        counter += counters.get(&format_point(row_count - 1, j)).unwrap_or(&0);
    }

    println!("counter: {}", counter);
}

fn format_entry(entry: &Entry) -> String {
    String::from(format!("{}-{}", entry.i, entry.j))
}

fn format_point(i: usize, j: usize) -> String {
    String::from(format!("{}-{}", i, j))
}