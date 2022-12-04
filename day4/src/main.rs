use regex::Regex;
use std::fs;

fn main() {
    let raw = fs::read_to_string("./inputs/day4.txt").unwrap();
    let input: Vec<&str> = raw.lines().collect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn parse_ranges(input: &Vec<&str>) -> Vec<(u32, u32, u32, u32)> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();

    input
        .iter()
        .map(|line| {
            let cap = re.captures(line).unwrap();

            let start_a: u32 = cap[1].parse().unwrap();
            let start_b: u32 = cap[3].parse().unwrap();
            let end_a: u32 = cap[2].parse().unwrap();
            let end_b: u32 = cap[4].parse().unwrap();

            (start_a, end_a, start_b, end_b)
        })
        .collect()
}

fn part_one(input: &Vec<&str>) -> usize {
    parse_ranges(input)
        .iter()
        .filter(|(start_a, end_a, start_b, end_b)| {
            (start_a >= start_b && end_a <= end_b) || (start_b >= start_a && end_b <= end_a)
        })
        .count()
}

fn part_two(input: &Vec<&str>) -> usize {
    parse_ranges(input)
        .iter()
        .filter(|(start_a, end_a, start_b, end_b)| {
            (start_a >= start_b && start_a <= end_b) || (start_b >= start_a && start_b <= end_a)
        })
        .count()
}
