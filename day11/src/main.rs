use crate::monkey::Monkey;
use std::fs;

mod monkey;

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    let raw = fs::read_to_string("./inputs/day11.txt").unwrap();

    let monkeys: Vec<Monkey> = raw
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|input| Monkey::from(input))
        .collect();

    println!("Part 1: {}", part_one(&monkeys));
    println!("Part 2: {}", part_two(&monkeys));
}

fn monkey_business(monkeys: &mut Vec<Monkey>, rounds: usize, decrease_worry: bool) -> usize {
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];

    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.test.divisor)
        .fold(1, |acc, x| lcm(acc, x));

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            inspections[i] += monkeys[i].items.len();
            while monkeys[i].items.len() > 0 {
                let new = if decrease_worry {
                    monkeys[i].operation.run(monkeys[i].items[0]) / 3
                } else {
                    monkeys[i].operation.run(monkeys[i].items[0]) % lcm
                };
                let throw_to = monkeys[i].test.run(new);
                monkeys[throw_to].items.push(new);
                monkeys[i].items.remove(0);
            }
        }
    }

    inspections.sort();
    inspections.iter().rev().take(2).fold(1, |acc, x| acc * x)
}

fn part_one(monkeys: &Vec<Monkey>) -> usize {
    let mut fresh_monkeys = (*monkeys).clone();
    monkey_business(&mut fresh_monkeys, 20, true)
}

fn part_two(monkeys: &Vec<Monkey>) -> usize {
    let mut fresh_monkeys = (*monkeys).clone();
    monkey_business(&mut fresh_monkeys, 10000, false)
}
