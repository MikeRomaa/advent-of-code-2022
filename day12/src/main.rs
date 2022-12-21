use crate::height_map::HeightMap;
use std::fs;

mod height_map;

fn main() {
    let raw = fs::read_to_string("./inputs/day12.txt").unwrap();

    let map = HeightMap::from(raw);

    println!("Part 1: {}", part_one(&map));
    println!("Part 2: {}", part_two(&map));
}

fn shortest_path_length(map: &HeightMap, start: (usize, usize)) -> usize {
    // 2D array of same size as `map` filled with max u8 values,
    // start location will have a distance of 0
    let mut distances = vec![vec![usize::MAX; map.heights[0].len()]; map.heights.len()];
    distances[start.1][start.0] = 0;

    // List of all coordinates in the map
    let mut queue: Vec<(usize, usize)> = map
        .heights
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, _)| (x, y)))
        .collect();

    while queue.len() > 0 {
        // Find and remove location with smallest distance
        queue.sort_by_key(|&loc| distances[loc.1][loc.0]);
        let current = queue.remove(0);

        // If the smallest distance is usize::MAX (default value),
        // it is not possible to reach the target from the given start location
        if distances[current.1][current.0] == usize::MAX {
            return usize::MAX;
        }

        if current == map.end {
            break;
        }

        for neighbor in map.get_neighbors(current, false) {
            let distance = distances[current.1][current.0] + 1;
            if distance < distances[neighbor.1][neighbor.0] {
                distances[neighbor.1][neighbor.0] = distance;
            }
        }
    }

    let mut current = map.end;
    let mut steps = 0;

    // Retrace path by finding shortest distance from valid neighbors
    while current != start {
        current = *map
            .get_neighbors(current, true)
            .iter()
            .min_by_key(|&loc| distances[loc.1][loc.0])
            .unwrap();
        steps += 1;
    }

    steps
}

fn part_one(map: &HeightMap) -> usize {
    shortest_path_length(map, map.start)
}

fn part_two(map: &HeightMap) -> usize {
    let candidates: Vec<(usize, usize)> = map
        .heights
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &value)| value == 0)
                .map(|(x, _)| (x, y))
                .collect::<Vec<_>>()
        })
        .collect();

    candidates
        .iter()
        .map(|&loc| shortest_path_length(map, loc))
        .min()
        .unwrap()
}
