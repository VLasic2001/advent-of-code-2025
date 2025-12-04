use std::fs;

fn main() {
    let mut dial_state = 50;

    let mut counter = 0;

    let rotation_string = fs::read_to_string("input.txt").expect("error with read");

    let rotations = rotation_string.split_whitespace();

    for rotation in rotations {
        println!("{dial_state} {counter} {rotation}");
        move_dial(
            &mut dial_state,
            &rotation[..1],
            &rotation[1..].parse().expect("not a number"),
            &mut counter,
        );
    }

    println!("{dial_state} {counter}");
}

fn move_dial(dial: &mut i32, direction: &str, amount: &i32, counter: &mut i32) {
    if direction.contains("L") {
        //5-210=-205
        let times_passed = -(*dial - *amount) / 100;
        let new_dial = (*dial - *amount + (times_passed+1)*100 ) % 100;
        if amount >= dial {
            *counter += times_passed;
            if *dial != 0 {
                *counter+=1;
            }
        }
        *dial = new_dial;
    } else if direction.contains("R") {
        let times_passed = (*dial + *amount) / 100;
        let new_dial = (*dial + *amount) % 100;
        *counter += times_passed;
        *dial = new_dial;
    } else {
        println!("Error");
    }
}
