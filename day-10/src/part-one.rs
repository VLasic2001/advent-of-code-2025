use std::{collections::VecDeque, fs};

#[derive(Debug)]
struct QueueEntry {
    steps: u32,
    current_state: u32,
    final_state: u32,
    step_to_execute: u32,
    visited_steps: Vec<u32>,
}

struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    _joltages: Vec<i32>,
}

impl Machine {
    pub fn new(data: &str) -> Self {
        let lights_string: &str = extract_bracketed(data, '[', ']');
        let lights = lights_string
            .chars()
            .map(|c| match c {
                '.' => '0',
                '#' => '1',
                _ => ' ',
            })
            .collect::<String>();
        let light_str = u32::from_str_radix(&lights, 2).unwrap();

        let buttons: Vec<u32> = data
            .split(' ')
            .filter(|b| b.contains('('))
            .map(|b| {
                calculate_xor_binary(
                    b.trim_matches(|c| c == '(' || c == ')'),
                    lights_string.len(),
                )
            })
            .collect();

        return Machine {
            buttons: buttons,
            lights: light_str,
            _joltages: vec![0],
        };
    }
}

fn main() {
    let machines: Vec<Machine> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|m| Machine::new(m))
        .collect();

    let mut results: Vec<u32> = Vec::new();

    for machine in machines {
        println!("krenilo - {}", machine.lights);
        let mut queue: VecDeque<QueueEntry> = VecDeque::new();

        for button in &machine.buttons {
            queue.push_back(QueueEntry {
                steps: 0,
                current_state: 0,
                final_state: machine.lights,
                step_to_execute: *button,
                visited_steps: vec![],
            });
        }

        'queue: while queue.len() != 0 {
            let mut entry = queue.pop_front().unwrap();

            let next_state = entry.current_state ^ entry.step_to_execute;

            entry.steps += 1;
            entry.visited_steps.push(entry.step_to_execute);

            if next_state == entry.final_state {
                results.push(entry.steps);
                break 'queue;
            }

            for button in &machine.buttons {
                if !entry.visited_steps.contains(&button) {
                    queue.push_back(QueueEntry {
                        steps: entry.steps,
                        current_state: next_state,
                        final_state: entry.final_state,
                        step_to_execute: *button,
                        visited_steps: entry.visited_steps.clone(),
                    });
                }
            }
        }
    }

        let mut total = 0;
        for res in &results {
            total += *res;
            println!("res - {}", *res);
        }
        println!("total - {}", total);
}

fn extract_bracketed(s: &str, start_char: char, end_char: char) -> &str {
    let start = s.find(start_char).unwrap() + 1;
    let end = s[start..].find(end_char).unwrap() + start;
    &s[start..end]
}

fn calculate_xor_binary(s: &str, len: usize) -> u32 {
    let mut bin_vec = vec!['0'; len];

    for num in s.split(',') {
        bin_vec[num.parse::<u32>().unwrap() as usize] = '1';
    }

    let bin_str = bin_vec.iter().collect::<String>();

    u32::from_str_radix(&bin_str, 2).unwrap()
}
