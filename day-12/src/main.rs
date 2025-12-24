use std::fs;

#[derive(Debug)]
struct Input {
    size: u128,
    shapes: Vec<u16>,
    max_shapes: u128,
}

impl Input {
    fn new(data: &str) -> Self {
        let splits: Vec<&str> = data.split(": ").collect();
        let size: Vec<&str> = splits[0].split("x").collect();
        let shapes: Vec<&str> = splits[1].split(" ").collect();
        let w: u128 = size[0].parse().unwrap();
        let h: u128 = size[1].parse().unwrap();
        Input { size: w*h, shapes: shapes.iter().map(|s| s.parse().unwrap()).collect(), max_shapes: (w/3)*(h/3) }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let start = input.find("40").unwrap();
    let inputs: Vec<Input> = (&input[start..]).lines().map(|i| Input::new(i)).collect();
    let shapes = vec![7, 5, 7, 7, 6, 7];

    let mut cant_fit_count = 0;
    let mut always_fit_count = 0;
    let mut other = 0;

    for inp in &inputs {
        let mut min_shapes_size: u128 = 0;
        let mut max_shapes_size: u128 = 0;
        for (i, shape) in inp.shapes.iter().enumerate() {
            min_shapes_size += (*shape*shapes[i]) as u128;
            max_shapes_size += (*shape) as u128;
        }

        if min_shapes_size > inp.size {
            cant_fit_count += 1;
            continue;
        }

        if max_shapes_size <= inp.max_shapes {
            always_fit_count += 1;
            continue;
        }

        other += 1;
    }

    println!("len: {}", inputs.len());
    println!("cant: {}", cant_fit_count);
    println!("always: {}", always_fit_count);
    println!("other: {}", other);
}
