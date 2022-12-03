use std::fs;

fn main() {
    let raw = fs::read_to_string("./inputs/day3.txt").unwrap();
    let input: Vec<&str> = raw.lines().collect();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn eval_priority(items: Vec<char>) -> usize {
    items.iter().fold(0, |acc, x| {
        let ascii = *x as usize;
        if ascii > 96 {
            // Lowercase starts at 97
            acc + ascii - 96
        } else {
            // Uppercase starts at 65
            acc + ascii - 64 + 26
        }
    })
}

fn part_one(input: &Vec<&str>) -> usize {
    let mut shared: Vec<char> = Vec::new();

    for &line in input {
        let (a, b) = line.split_at(line.len() / 2);

        let contents_a: Vec<char> = a.chars().collect();
        let contents_b: Vec<char> = b.chars().collect();

        for char in contents_a {
            if contents_b.contains(&char) {
                shared.push(char);
                break;
            }
        }
    }

    eval_priority(shared)
}

fn part_two(input: &Vec<&str>) -> usize {
    let mut shared: Vec<char> = Vec::new();

    for group in input.chunks(3) {
        let contents_a: Vec<char> = group[0].chars().collect();
        let contents_b: Vec<char> = group[1].chars().collect();
        let contents_c: Vec<char> = group[2].chars().collect();

        for char in contents_a {
            if contents_b.contains(&char) {
                if contents_c.contains(&char) {
                    shared.push(char);
                    break;
                }
            }
        }
    }

    eval_priority(shared)
}
