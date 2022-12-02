use std::fs;

fn main() {
    let raw = fs::read_to_string("./inputs/day2.txt").unwrap();
    let input: Vec<&str> = raw.lines().collect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &Vec<&str>) -> usize {
    let mut score = 0;

    for &line in input {
        match line {
            "A X" => score += 1 + 3, // Rock     Rock     Tie
            "B X" => score += 1 + 0, // Paper    Rock     Loss
            "C X" => score += 1 + 6, // Scissors Rock     Win
            "A Y" => score += 2 + 6, // Rock     Paper    Win
            "B Y" => score += 2 + 3, // Paper    Paper    Tie
            "C Y" => score += 2 + 0, // Scissors Paper    Loss
            "A Z" => score += 3 + 0, // Rock     Scissors Loss
            "B Z" => score += 3 + 6, // Paper    Scissors Win
            "C Z" => score += 3 + 3, // Scissors Scissors Tie
            _ => (),
        }
    }

    score
}

fn part_two(input: &Vec<&str>) -> usize {
    let mut score = 0;

    for &line in input {
        match line {
            "A X" => score += 3 + 0, // Rock     Loss Scissors
            "B X" => score += 1 + 0, // Paper    Loss Rock
            "C X" => score += 2 + 0, // Scissors Loss Paper
            "A Y" => score += 1 + 3, // Rock     Tie  Rock
            "B Y" => score += 2 + 3, // Paper    Tie  Paper
            "C Y" => score += 3 + 3, // Scissors Tie  Scissors
            "A Z" => score += 2 + 6, // Rock     Win  Paper
            "B Z" => score += 3 + 6, // Paper    Win  Scissors
            "C Z" => score += 1 + 6, // Scissors Win  Rock
            _ => (),
        }
    }

    score
}
