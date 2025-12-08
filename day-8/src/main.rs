use std::fs;

struct Box {
    x: f64,
    y: f64,
    z: f64,
}

struct Distance {
    first_id: usize,
    second_id: usize,
    distance: f64,
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("error reading from file");

    let lines: Vec<&str> = input.lines().collect();
    let mut boxes: Vec<Box> = Vec::new();
    let mut distances: Vec<Distance> = Vec::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();

    let mut unused_boxes: Vec<usize> = Vec::from_iter(0..lines.len());

    read_boxes(lines, &mut boxes);

    for i in 0..boxes.len() {
        for j in 0..i {
            let distance = Distance {
                first_id: i,
                second_id: j,
                distance: ((boxes[i].x - boxes[j].x).powi(2)
                    + (boxes[i].y - boxes[j].y).powi(2)
                    + (boxes[i].z - boxes[j].z).powi(2))
                .sqrt(),
            };

            distances.push(distance);
        }
    }

    distances.sort_unstable_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    // for i in 0..1000 {
    //     let mut new_circuit: Vec<usize> = Vec::new();
    //     let mut new_circuits: Vec<Vec<usize>> = Vec::new();
    //     new_circuit.push(distances[i].first_id);
    //     new_circuit.push(distances[i].second_id);
    //     let unused_index_one = unused_boxes.iter().position(|b| *b == distances[i].first_id);
    //     if unused_index_one.is_some() {
    //         unused_boxes.swap_remove(unused_index_one.unwrap());
    //         if unused_boxes.len() == 0 {
    //             break;
    //         }
    //     }
    //     let unused_index_two = unused_boxes.iter().position(|b| *b == distances[i].second_id);
    //     if unused_index_two.is_some() {
    //         unused_boxes.swap_remove(unused_index_two.unwrap());
    //         if unused_boxes.len() == 0 {
    //             break;
    //         }
    //     }
    //     for circuit in circuits.iter() {
    //         if circuit.contains(&distances[i].first_id) || circuit.contains(&distances[i].second_id) {
    //             for c in circuit {
    //                 if !new_circuit.contains(c) {
    //                     new_circuit.push(*c);
    //                 }
    //             }
    //         } else {
    //             new_circuits.push(circuit.clone());
    //         }
    //     }
    //     new_circuits.push(new_circuit.clone());
    //     circuits = new_circuits;
    // }

    let mut second_last_id = 0;
    let mut last_id = 0;

 for i in 0..distances.len() {
        let mut new_circuit: Vec<usize> = Vec::new();
        let mut new_circuits: Vec<Vec<usize>> = Vec::new();
        new_circuit.push(distances[i].first_id);
        new_circuit.push(distances[i].second_id);
        let unused_index_one = unused_boxes.iter().position(|b| *b == distances[i].first_id);
        if unused_index_one.is_some() {
            unused_boxes.swap_remove(unused_index_one.unwrap());
        }
        let unused_index_two = unused_boxes.iter().position(|b| *b == distances[i].second_id);
        if unused_index_two.is_some() {
            unused_boxes.swap_remove(unused_index_two.unwrap());
        }
        for circuit in circuits.iter() {
            if circuit.contains(&distances[i].first_id) || circuit.contains(&distances[i].second_id) {
                for c in circuit {
                    if !new_circuit.contains(c) {
                        new_circuit.push(*c);
                    }
                }
            } else {
                new_circuits.push(circuit.clone());
            }
        }
        new_circuits.push(new_circuit.clone());
        circuits = new_circuits;

        if circuits.len() == 1 && unused_boxes.len() == 0{
                second_last_id = distances[i].second_id;
                last_id = distances[i].first_id;
                break;
        }
    }

    println!("len after {}", unused_boxes.len());
    
    println!("cir len {}", circuits.len());

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut counter = 1;

    // for i in 0..3 {
    //     counter *= circuits[i].len();
    // }

    println!("counter {}", counter);

    println!("second {} {:?}",second_last_id, boxes[second_last_id].x);
    println!("last {} {:?}",last_id, boxes[last_id].x);

    println!("total {}", boxes[second_last_id].x * boxes[last_id].x);
}

fn read_boxes(lines: Vec<&str>, boxes: &mut Vec<Box>) {
    for line in lines.iter() {
        let coords: Vec<f64> = line
            .split(",")
            .map(|n| n.parse().expect("error in parse"))
            .collect();

        let r#box = Box {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        };

        boxes.push(r#box);
    }
}
