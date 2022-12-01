use std::cmp::max;
use std::fs;

fn main() {
    let raw = fs::read_to_string("./inputs/day1.txt").unwrap();
    let input: Vec<&str> = raw.lines().collect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &Vec<&str>) -> usize {
    let mut max_calories = 0;
    let mut current_calories = 0;

    for line in input {
        if line.is_empty() {
            max_calories = max(max_calories, current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<usize>().unwrap();
        }
    }

    max_calories
}

fn part_two(input: &Vec<&str>) -> usize {
    let mut calorie_sums: Vec<usize> = Vec::new();
    let mut current_calories = 0;

    for line in input {
        if line.is_empty() {
            calorie_sums.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<usize>().unwrap();
        }
    }

    calorie_sums.sort_by(|a, b| b.partial_cmp(a).unwrap());
    calorie_sums[0..3].iter().fold(0, |acc, x| acc + x)
}
