use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(data: &str) -> Self {
        let spl = data
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>();
        Point {
            x: spl[0] as usize,
            y: spl[1] as usize,
        }
    }
}

// #[derive(Debug)]
// struct Bound {
//     first: i64,
//     last: i64,
// }

fn main() {
    let points: Vec<Point> = fs::read_to_string("input.txt")
        .unwrap()
        .split_whitespace()
        .map(|l| Point::new(l))
        .collect();

    let mut max_x = 0;
    let mut max_y = 0;

    for point in &points {
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    let mut grid: Vec<Vec<char>> = Vec::new();

    for _row in 0..max_y + 1 {
        grid.push(vec!['.'; max_x + 1]);
    }

    for (i, point) in points.iter().enumerate() {
        grid[point.y][point.x] = '#';

        if i == 0 {
            continue;
        }

        connect_point(&mut grid, point, &points[i - 1]);
        if i == points.len() - 1 {
            connect_point(&mut grid, point, &points[0]);
        }
    }

    for (i, row) in grid.clone().iter().enumerate() {
        println!("fill - {}", i);
        if i == row.len() - 1 {
            continue;
        }

        let mut is_printing = false;

        for (j, _c) in row.iter().enumerate() {
            if j == row.len() - 1 {
                continue;
            }

            if grid[i][j] == '#' && grid[i][j + 1] == '.' && !is_printing {
                if j >= 2 {
                    if grid[i][j - 2] == '.' && grid[i][j - 1] == '#' {
                        continue;
                    }
                }
                is_printing = true;
            } else if grid[i][j] == '#' && is_printing {
                is_printing = false;
            } else if grid[i][j] == '.' && is_printing {
                grid[i][j] = '#';
            }
        }
    }

    // let mut bounds: Vec<Bound> = Vec::new();

    // for row in grid {
    // let bound = Bound {
    //     first: row
    //         .iter()
    //         .position(|&c| c == '#')
    //         .map(|i| i as i64)
    //         .unwrap_or(-1),
    //     last: row
    //         .iter()
    //         .rposition(|&c| c == '#')
    //         .map(|i| i as i64)
    //         .unwrap_or(-1),
    // };

    // bounds.push(bound);
    // }

    let mut max_tiles = 0;

    for (i, first) in points.iter().enumerate() {
        println!("i - {}", i);
        'bounds_loop: for (j, second) in points.iter().enumerate() {
            if i < j {
                continue;
            }

            let tiles;

            if check_points(&grid, first, second) {
                continue 'bounds_loop;
            }

            if first.y > second.y {
                if first.x > second.x {
                    tiles = (first.x - second.x + 1) * (first.y - second.y + 1);
                } else {
                    tiles = (second.x - first.x + 1) * (first.y - second.y + 1);
                }
            } else {
                if first.x > second.x {
                    tiles = (first.x - second.x + 1) * (second.y - first.y + 1);
                } else {
                    tiles = (second.x - first.x + 1) * (second.y - first.y + 1);
                }
            }

            if tiles > max_tiles {
                max_tiles = tiles;
            }
        }
    }

    println!("tiles {}", max_tiles);
}

fn connect_point(grid: &mut Vec<Vec<char>>, first: &Point, second: &Point) {
    if first.x == second.x {
        if first.y < second.y {
            for j in first.y..second.y {
                grid[j][first.x] = '#';
            }
        } else {
            for j in second.y..first.y {
                grid[j][first.x] = '#';
            }
        }
    } else {
        if first.x < second.x {
            for j in first.x..second.x {
                grid[first.y][j] = '#';
            }
        } else {
            for j in second.x..first.x {
                grid[first.y][j] = '#';
            }
        }
    }
}

fn check_points(grid: &Vec<Vec<char>>, first: &Point, second: &Point) -> bool {
    if first.x < second.x {
        for i in first.x..second.x {
            if grid[first.y][i] == '.' || grid[second.y][i] == '.' {
                return true;
            }
        }
    } else {
        for i in second.x..first.x {
            if grid[first.y][i] == '.' || grid[second.y][i] == '.' {
                return true;
            }
        }
    }
    if first.y < second.y {
        for i in first.y..second.y {
            if grid[i][first.x] == '.' || grid[i][second.x] == '.' {
                return true;
            }
        }
    } else {
        for i in second.y..first.y {
            if grid[i][first.x] == '.' || grid[i][second.x] == '.' {
                return true;
            }
        }
    }

    false
}
