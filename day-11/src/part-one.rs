use std::{collections::VecDeque, fs};

#[derive(Debug, Clone)]
struct State {
    input: String,
    outputs: Vec<String>,
}

struct QueueEntry {
    current_state: String,
    visited: Vec<String>
}

impl State {
    fn new(data: &str) -> Self {
        let splits: Vec<&str> = data.split(":").collect();
        State {
            input: String::from(splits[0]),
            outputs: splits[1]
                .split(" ")
                .filter(|s| *s != "")
                .map(|s| String::from(s))
                .collect(),
        }
    }
}

fn main() {
    let states: Vec<State> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| State::new(l))
        .collect();

    let mut queue: VecDeque<QueueEntry> = VecDeque::new();
    queue.push_back(QueueEntry { current_state: String::from("you"), visited: vec![] });

    let mut count = 0;

    loop { 
        let entry_option = queue.pop_front();

        if entry_option.is_none() { break; }

        let entry = entry_option.unwrap();
        let current_state = states.iter().find(|s| s.input == entry.current_state).unwrap().clone();

        for out in current_state.outputs {
            if out == "out" {
                count += 1;
                continue;
            } 

            let new_state = states.iter().find(|s| s.input == out).unwrap().clone();
            queue.push_back(QueueEntry {current_state: new_state.input, visited: Vec::new()});
        }
    }

    println!("{}", count);
}
