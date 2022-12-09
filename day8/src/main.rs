use std::cmp::min;
use std::fs;

fn main() {
    let raw = fs::read_to_string("./inputs/day8.txt").unwrap();
    let input: Vec<Vec<u8>> = raw
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(matrix: &Vec<Vec<u8>>) -> usize {
    let mut count = 0;

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            // Edge tree, immediately count as visible
            if y == 0 || y == matrix.len() - 1 || x == 0 || x == matrix[0].len() - 1 {
                count += 1;
                continue;
            }

            let max_left = *matrix[y][..x].iter().max().unwrap();
            let max_right = *matrix[y][x + 1..].iter().max().unwrap();
            let max_top = matrix[..y].iter().map(|row| row[x]).max().unwrap();
            let max_bottom = matrix[y + 1..].iter().map(|row| row[x]).max().unwrap();

            if matrix[y][x] > min(max_left, min(max_right, min(max_top, max_bottom))) {
                count += 1;
            }
        }
    }

    count
}

fn part_two(matrix: &Vec<Vec<u8>>) -> usize {
    let mut scores: Vec<usize> = Vec::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            // Edge tree, immediately discard since score will be 0
            if y == 0 || y == matrix.len() - 1 || x == 0 || x == matrix[0].len() - 1 {
                continue;
            }

            let height = matrix[y][x];

            let score_top = 1 + matrix[..y]
                .iter()
                .map(|row| row[x])
                .rev()
                .position(|val| val >= height)
                .unwrap_or(matrix[..y].len() - 1);

            let score_left = 1 + matrix[y][..x]
                .iter()
                .rev()
                .position(|&val| val >= height)
                .unwrap_or(matrix[y][..x].len() - 1);

            let score_bottom = 1 + matrix[y + 1..]
                .iter()
                .map(|row| row[x])
                .position(|val| val >= height)
                .unwrap_or(matrix[y + 1..].len() - 1);

            let score_right = 1 + matrix[y][x + 1..]
                .iter()
                .position(|&val| val >= height)
                .unwrap_or(matrix[0][x + 1..].len() - 1);

            scores.push(score_top * score_left * score_bottom * score_right);
        }
    }

    *scores.iter().max().unwrap()
}
