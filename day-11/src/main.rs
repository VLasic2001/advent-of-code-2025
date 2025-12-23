use std::collections::hash_map::Entry;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

const START: &str = "svr";
const FIRST_CHECKPOINT: &str = "fft";
const SECOND_CHECKPOINT: &str = "dac";
const END: &str = "out";

#[derive(Debug, Clone)]
struct State {
    input: String,
    outputs: Vec<String>,
}

#[derive(Debug, Clone)]
struct DPState {
    before_c1: u64,
    after_c1: u64,
    after_c1_c2: u64,
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

    let mut states_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    let mut dp: HashMap<String, DPState> = HashMap::new();
    for state in &states {
        states_hashmap.insert(state.input.clone(), state.outputs.clone());
        dp.insert(
            String::from(state.input.clone()),
            DPState {
                before_c1: 0,
                after_c1: 0,
                after_c1_c2: 0,
            },
        );
    }
    dp.insert(
        START.to_string(),
        DPState {
            before_c1: 1,
            after_c1: 0,
            after_c1_c2: 0,
        },
    );

    let mut out_states = DPState{
            before_c1: 0,
            after_c1: 0,
            after_c1_c2: 0,
        };

    let topo_states = topo_sort(states);

    for topo_state in topo_states {
        if topo_state == END { continue; }
        
        let dp_state = dp.get(&topo_state).unwrap().clone();
        let outputs = states_hashmap.get(&topo_state).unwrap();

        for out in outputs {
            if out == END { 
                out_states.after_c1_c2 += dp_state.after_c1_c2;
                continue;
            }

            let dp_state_out = dp.get_mut(out).unwrap();

            if dp_state.before_c1 > 0 {
                if out == FIRST_CHECKPOINT {
                    dp_state_out.after_c1 += dp_state.before_c1;
                } else if out != SECOND_CHECKPOINT {
                    dp_state_out.before_c1 += dp_state.before_c1;
                }
            }

            if dp_state.after_c1 > 0 {
                if out == SECOND_CHECKPOINT {
                    dp_state_out.after_c1_c2 += dp_state.after_c1;
                } else {
                    dp_state_out.after_c1 += dp_state.after_c1;
                }
            }

            if dp_state.after_c1_c2 > 0 {
                    dp_state_out.after_c1_c2 += dp_state.after_c1_c2;
            }
        }
    }

    println!("{:?}", out_states);
}

fn topo_sort(states: Vec<State>) -> Vec<String> {
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    let mut indeg: HashMap<String, i32> = HashMap::new();

    for state in states {
        match adj.entry(state.input) {
            Entry::Vacant(e) => {
                e.insert(state.outputs.clone());
            }
            Entry::Occupied(mut e) => {
                e.get_mut().append(&mut state.outputs.clone());
            }
        }
        for out in state.outputs {
            match indeg.entry(String::from(out)) {
                Entry::Vacant(e) => {
                    e.insert(1);
                }
                Entry::Occupied(mut e) => {
                    *e.get_mut() += 1;
                }
            }
        }
    }

    let mut deque: VecDeque<String> = VecDeque::new();
    let mut topo: Vec<String> = vec![];
    deque.push_back(String::from(START));

    while let Some(entry) = deque.pop_front() {
        topo.push(entry.clone());

        if let Some(outs) = adj.get(&entry) {
            for out in outs {
                if let Some(value) = indeg.get_mut(&String::from(out)) {
                    *value -= 1;
                    if *value == 0 {
                        deque.push_back(out.clone());
                    }
                }
            }
        }
    }

    topo
}
