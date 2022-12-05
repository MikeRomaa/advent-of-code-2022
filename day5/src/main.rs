use regex::Regex;
use std::fs;

fn main() {
    let raw = fs::read_to_string("./inputs/day5.txt").unwrap();
    let mut input: Vec<&str> = raw.lines().collect();

    let split = input.iter().position(|&item| item.is_empty()).unwrap();
    let (crates, instructions) = input.split_at_mut(split + 1);

    // First line is an empty string,
    // second is the stack labels,
    // rest is the crates from the bottom up
    crates.reverse();

    let stacks = Regex::new(r"\d").unwrap().find_iter(crates[1]).count();

    let state_re = Regex::new(r"\[(.)]|\s{3,4}").unwrap();

    let mut state: Vec<Vec<&str>> = Vec::new();
    for i in 0..stacks {
        state.push(
            crates[2..]
                .iter()
                .filter_map(|&line| match state_re.captures_iter(line).nth(i) {
                    Some(capture) => match capture.get(1) {
                        Some(str) => Some(str.as_str()),
                        None => None,
                    },
                    None => None,
                })
                .collect(),
        );
    }

    let instruction_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let parsed_instructions: Vec<(usize, usize, usize)> = Vec::from(instructions)
        .iter()
        .map(|&instruction| {
            let c = instruction_re.captures(instruction).unwrap();
            (
                c.get(1).unwrap().as_str().parse().unwrap(),
                c.get(2).unwrap().as_str().parse().unwrap(),
                c.get(3).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();

    println!("Part 1: {}", part_one(&state, &parsed_instructions));
    println!("Part 2: {}", part_two(&state, &parsed_instructions));
}

fn part_one(state: &Vec<Vec<&str>>, instructions: &Vec<(usize, usize, usize)>) -> String {
    let mut new_state = state.clone();

    for (num, from, to) in instructions {
        for _ in 0..*num {
            let moved = new_state[from - 1].pop().unwrap();
            new_state[to - 1].push(moved);
        }
    }

    new_state
        .iter()
        .fold(String::new(), |str, stack| str + stack.last().unwrap())
}

fn part_two(state: &Vec<Vec<&str>>, instructions: &Vec<(usize, usize, usize)>) -> String {
    let mut new_state = state.clone();

    for (num, from, to) in instructions {
        let mut temp: Vec<&str> = Vec::new();
        for _ in 0..*num {
            let moved = new_state[from - 1].pop().unwrap();
            temp.push(moved);
        }
        while temp.len() > 0 {
            let moved = temp.pop().unwrap();
            new_state[to - 1].push(moved);
        }
    }

    new_state
        .iter()
        .fold(String::new(), |str, stack| str + stack.last().unwrap())
}
