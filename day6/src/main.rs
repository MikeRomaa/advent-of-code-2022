use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day6.txt").unwrap();

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn find_n_distinct_chars(input: &String, n: usize) -> usize {
    let chars = input.chars().collect::<Vec<_>>();

    // Iterate over overlapping groups of n chars
    for (i, group) in chars.windows(n).enumerate() {
        let mut group_vec: Vec<&char> = group.iter().collect();
        group_vec.sort();
        group_vec.dedup();

        // If group has n distinct letters, return index of last letter
        if group_vec.len() == n {
            return i + n;
        }
    }

    0
}

fn part_one(input: &String) -> usize {
    find_n_distinct_chars(input, 4)
}

fn part_two(input: &String) -> usize {
    find_n_distinct_chars(input, 14)
}
