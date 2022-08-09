use std::{collections::HashMap};

use regex::{Regex, Captures};

fn main() {
    let content = std::fs::read_to_string("./src/day_6/input.txt")
        .expect("Unable to read input");

    let matcher = Regex::new(r"(.+)\s(\d+),(\d+)\sthrough\s(\d+),(\d+)")
        .unwrap();

    let result1: usize = {
        let mut lights: HashMap<(usize, usize), bool> = HashMap::with_capacity(1_000_000);

        content
            .lines()
            .map(|it| matcher.captures(it))
            .filter_map(Step::create)
            .for_each(|it| first_puzzle(&it, &mut lights));

        lights
            .values()
            .filter(|it| **it)
            .count()
    };

    let result2: i32 = {
        let mut lights: HashMap<(usize, usize), i32> = HashMap::with_capacity(1_000_000);

        content
            .lines()
            .map(|it| matcher.captures(it))
            .filter_map(Step::create)
            .for_each(|it| second_puzzle(&it, &mut lights));

        lights
            .values()
            .sum()
    };

    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);

}

#[derive(Debug)]
enum Action { On, Off, Toggle }

#[derive(Debug)]
struct Step {
    action: Action,
    start: (usize, usize),
    end: (usize, usize),
}

impl Step {
    fn create(captures: Option<Captures>) -> Option<Step> {
        let matches = captures?;
    
        let action = match &matches[1] {
            "turn on" => Action::On,
            "turn off" => Action::Off,
            "toggle" => Action::Toggle,
            _ => { return None }
        };
    
        if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (
            matches[2].parse::<usize>(), 
            matches[3].parse::<usize>(),
            matches[4].parse::<usize>(),
            matches[5].parse::<usize>()
        ) {
            Some(Step { action, start: (x1, y1), end: (x2, y2) })
        } else {
            None
        }
    }
}

fn first_puzzle(step: &Step, lights: &mut HashMap<(usize, usize), bool>) {
    let (row_start, columm_start) = step.start;
    let (row_end, column_end) = step.end;

    for r in row_start..=row_end {
        for c in columm_start..=column_end {
            let position = (r, c);

            if let Some(value) = lights.get(&position).or(Some(&false)) {
                let next = match step.action {
                    Action::Off => false,
                    Action::On => true,
                    Action::Toggle => !value,
                };

                lights.insert(position, next);
            }
        }
    }
}

fn second_puzzle(step: &Step, lights: &mut HashMap<(usize, usize), i32>) {
    let (row_start, columm_start) = step.start;
    let (row_end, column_end) = step.end;

    for r in row_start..=row_end {
        for c in columm_start..=column_end {
            let position = (r, c);

            if let Some(value) = lights.get(&position).or(Some(&0)) {
                let next = match step.action {
                    Action::On => value + 1,
                    Action::Off => if value - 1 <= 0 { 0 } else { value - 1 },
                    Action::Toggle => value + 2,
                };

                lights.insert(position, next);
            }
        }
    }
}