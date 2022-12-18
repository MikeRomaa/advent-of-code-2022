use std::fs;
use std::slice::Iter;

enum Instruction {
    Add(i32, u8),
    Noop,
}

struct Program<'a> {
    input: Iter<'a, &'a str>,
    instruction: Instruction,
    register: i32,
}

impl<'a> Program<'a> {
    fn new(input: &'a Vec<&str>) -> Self {
        Self {
            input: input.iter(),
            instruction: Instruction::Noop,
            register: 1,
        }
    }
}

impl<'a> Iterator for Program<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.instruction {
            Instruction::Add(value, cycles) => {
                if cycles == 0 {
                    self.register += value;
                } else {
                    self.instruction = Instruction::Add(value, cycles - 1);
                    return Some(self.register);
                }
            }
            Instruction::Noop => (),
        }

        match self.input.next() {
            Some(instruction) => {
                let args: Vec<&str> = instruction.split(" ").collect();

                self.instruction = match args[0] {
                    "addx" => Instruction::Add(args[1].parse().unwrap(), 1),
                    "noop" => Instruction::Noop,
                    _ => panic!("Invalid instruction!"),
                };

                Some(self.register)
            }
            None => None,
        }
    }
}

fn main() {
    let raw = fs::read_to_string("./inputs/day10.txt").unwrap();
    let input: Vec<&str> = raw.lines().collect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2:\n{}", part_two(&input));
}

fn part_one(input: &Vec<&str>) -> i32 {
    let program = Program::new(input);
    let mut strengths: Vec<i32> = Vec::new();

    program.into_iter().enumerate().for_each(|(i, ax)| {
        strengths.push((i + 1) as i32 * ax);
    });

    [20, 60, 100, 140, 180, 220]
        .iter()
        .fold(0, |acc, cycle| acc + strengths[cycle - 1])
}

fn part_two(input: &Vec<&str>) -> String {
    let program = Program::new(input);
    let mut out: Vec<char> = Vec::new();
    let mut position: usize = 0;

    program.into_iter().for_each(|ax| {
        let line_index = (position % 40) as i32;
        if line_index == ax || line_index == ax - 1 || line_index == ax + 1 {
            out.push('#');
        } else {
            out.push('.');
        }
        position += 1;
    });

    out.iter()
        .enumerate()
        .fold(String::new(), |acc, (i, char)| {
            if i % 40 == 0 {
                format!("{acc}\n{char}")
            } else {
                format!("{acc}{char}")
            }
        })
}
