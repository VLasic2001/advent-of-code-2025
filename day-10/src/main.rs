use good_lp::{default_solver, variable, Expression, ProblemVariables, Solution, SolverModel};
use std::fs;

#[derive(Debug)]
struct Equation {
    coeffs: Vec<usize>,
    eq: u16,
}

struct Machine {
    len: usize,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u16>,
}

impl Machine {
    pub fn new(data: &str) -> Self {
        let lights_string: &str = extract_bracketed(data, '[', ']');

        let buttons: Vec<Vec<usize>> = data
            .split(' ')
            .filter(|b| b.contains('('))
            .map(|b| calculate_increment_vec(b.trim_matches(|c| c == '(' || c == ')')))
            .collect();

        let joltages: Vec<u16> = extract_bracketed(data, '{', '}')
            .split(',')
            .map(|j| j.parse().unwrap())
            .collect();

        return Machine {
            len: lights_string.len(),
            buttons: buttons,
            joltages: joltages,
        };
    }
}

fn main() {
    let machines: Vec<Machine> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|m| Machine::new(m))
        .collect();

    let mut results: Vec<i32> = Vec::new();

    for (j, machine) in machines.iter().enumerate() {
        results.push(solve_machine(machine));
    }

    let mut total = 0;
    for res in &results {
        total += *res;
    }
    println!("total - {}", total);
}

fn extract_bracketed(s: &str, start_char: char, end_char: char) -> &str {
    let start = s.find(start_char).unwrap() + 1;
    let end = s[start..].find(end_char).unwrap() + start;
    &s[start..end]
}

fn calculate_increment_vec(s: &str) -> Vec<usize> {
    let mut bin_vec = Vec::new();

    for num in s.split(',') {
        bin_vec.push(num.parse::<u32>().unwrap() as usize);
    }

    bin_vec
}

fn solve_machine(machine: &Machine) -> i32 {
    let mut equations: Vec<Equation> = Vec::new();

    for (i, jolt) in machine.joltages.iter().enumerate() {
        let mut buttons: Vec<usize> = Vec::new();

        for (j, button) in machine.buttons.iter().enumerate() {
            if button.contains(&i) {
                buttons.push(j);
            }
        }

        equations.push(Equation {
            coeffs: buttons,
            eq: *jolt,
        });
    }

    let mut vars = ProblemVariables::new();
    let xs: Vec<_> = (0..machine.buttons.len())
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    let objective: Expression = xs.iter().fold(Expression::from(0), |acc, &x| acc + x);
    let mut problem = vars.minimise(&objective).using(default_solver);

    for eq in equations {
        let expr: Expression = eq.coeffs.iter().map(|&i| xs[i]).sum();
        problem = problem.with(expr.eq(eq.eq as i32));
    }

    let solution = problem.solve().unwrap();

    solution.eval(&objective) as i32
}
