use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn main() {
    let raw = fs::read_to_string("./inputs/day7.txt").unwrap();
    let input: Vec<&str> = raw.lines().collect();

    let mut dir_sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut current_path = PathBuf::new();

    for line in input {
        let args: Vec<&str> = line.split(" ").collect();

        match args[0] {
            // command, we only care about cd
            "$" => match args[1] {
                "cd" => match args[2] {
                    ".." => {
                        current_path.pop();
                    }
                    _ => current_path.push(args[2]),
                },
                _ => (),
            },
            // directory name from ls stdout, we can ignore this
            "dir" => (),
            // must be a filesize, which we will parse and add to our HashMap
            _ => {
                let file_size: usize = args[0].parse().unwrap();
                // add to all parent directory sizes
                current_path.ancestors().for_each(|path| {
                    let current_size = dir_sizes.get(path).unwrap_or(&0);
                    dir_sizes.insert(PathBuf::from(path), current_size + file_size);
                })
            }
        }
    }

    println!("Part 1: {}", part_one(&dir_sizes));
    println!("Part 2: {}", part_two(&dir_sizes));
}

fn part_one(sizes: &HashMap<PathBuf, usize>) -> usize {
    sizes
        .values()
        .filter(|&&size| size <= 100000)
        .fold(0, |acc, &size| acc + size)
}

fn part_two(sizes: &HashMap<PathBuf, usize>) -> usize {
    let used = sizes.get(&PathBuf::from("/")).unwrap();
    let threshold = used - 40000000;

    *sizes
        .values()
        .filter(|&&size| size >= threshold)
        .min()
        .unwrap()
}
