use std::fs;

fn main() {
    let input_string = fs::read_to_string("inputb.txt").expect("error with read");

    let input: Vec<&str> = input_string.lines().collect();

    let mut ranges: Vec<&str> = Vec::new();
    let mut ids: Vec<&str> = Vec::new();

    let mut scanned_empty = false;

    for inp in input {
        if inp.len() == 0 {
            scanned_empty = true;
            continue;
        }
        if scanned_empty == false {
            ranges.push(inp);
        } else {
            ids.push(inp);
        }
    }

    let mut range_start_points: Vec<i64> = Vec::new();
    let mut range_end_points: Vec<i64> = Vec::new();

    for range in ranges {
        let (start, end) = split_range(range);
        range_start_points.push(start.parse().expect("error parsing start point"));
        range_end_points.push(end.parse().expect("error parsing end point"));
    }

    let mut fresh_counter = 0;

    for id in ids {
        let id_int: i64 = id.parse().expect("error parsing id");
        for (i, start_point) in range_start_points.iter().enumerate() {
            if id_int < *start_point {
                continue;
            }
            if range_end_points[i] >= id_int {
                fresh_counter += 1;
                break;
            }
        }
    }

    println!("counter - {}", fresh_counter);

    let mut starts: Vec<i64> = Vec::from(range_start_points);
    let mut ends: Vec<i64> = Vec::from(range_end_points);

    let mut exit: bool = false;

    loop {
        iter_over_ranges(&mut starts, &mut ends, &mut exit);
        if exit {
            break;
        }
    }

    let mut fresh_id_counter = 0;

    for i in 0..starts.len() {
        fresh_id_counter += ends[i] - starts[i] + 1;
    }

    println!("ids - {}", fresh_id_counter);
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

fn iter_over_ranges(starts: &mut Vec<i64>, ends: &mut Vec<i64>, exit: &mut bool) {
    let mut new_starts: Vec<i64> = Vec::new();
    let mut new_ends: Vec<i64> = Vec::new();

    let mut global_made_change = false;
    
    for i in 0..starts.len() {
        let mut made_change = false;
        if i == 0 {
            new_starts.push(starts[i]);
            new_ends.push(ends[i]);
            continue;
        }
        for j in 0..new_starts.len() {
            if starts[i] < new_starts[j] && ends[i] > new_ends[j] {
                new_starts[j] = starts[i];
                new_ends[j] = ends[i];
                made_change = true;
                global_made_change = true;
                continue;
            } else if starts[i] >= new_starts[j] && ends[i] <= new_ends[j] {
                made_change = true;
                global_made_change = true
            } else if starts[i] < new_starts[j] && ends[i] >= new_starts[j] {
                new_starts[j] = starts[i];
                made_change = true;
                global_made_change = true;
                continue;
            } else if starts[i] <= new_ends[j] && ends[i] > new_ends[j] {
                new_ends[j] = ends[i];
                made_change = true;
                global_made_change = true;
                continue;
            }
        }

        if !made_change {
            new_starts.push(starts[i]);
            new_ends.push(ends[i]);
            continue;
        }
    }

    if !global_made_change {
        *exit = true;
    }

    starts.clear();
    starts.append(&mut new_starts);
    ends.clear();
    ends.append(&mut new_ends);
}
