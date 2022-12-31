use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::ops::Range;

#[derive(Hash, PartialEq, Eq)]
struct Location {
    x: i32,
    y: i32,
}

fn main() {
    let raw = fs::read_to_string("./inputs/day15.txt").unwrap();

    let re = Regex::new(r"Sensor at x=(?P<x1>-?\d+), y=(?P<y1>-?\d+): closest beacon is at x=(?P<x2>-?\d+), y=(?P<y2>-?\d+)").unwrap();

    let mut sensors: Vec<Location> = Vec::new();
    let mut beacons: Vec<Location> = Vec::new();

    for group in re.captures_iter(&raw) {
        sensors.push(Location {
            x: group["x1"].parse().unwrap(),
            y: group["y1"].parse().unwrap(),
        });
        beacons.push(Location {
            x: group["x2"].parse().unwrap(),
            y: group["y2"].parse().unwrap(),
        });
    }

    println!("Part 1: {}", part_one(&sensors, &beacons));
    println!("Part 2: {}", part_two(&sensors, &beacons));
}

fn search_beacon(
    sensors: &Vec<Location>,
    beacons: &Vec<Location>,
    y: i32,
    x_range: Option<Range<i32>>,
) -> (u32, Vec<Range<i32>>) {
    let mut ranges: Vec<Range<i32>> = sensors
        .iter()
        .zip(beacons.iter())
        .filter_map(|(sensor, beacon)| {
            let radius = sensor.x.abs_diff(beacon.x) + sensor.y.abs_diff(beacon.y);
            let dy = sensor.y.abs_diff(y);

            // If we are within the search radius of the sensor, we can figure out the width of the
            // specific slice that we are looking at and represent its x coordinates as a range
            if dy <= radius {
                let width = radius - sensor.y.abs_diff(y);
                let start = sensor.x - width as i32;
                let end = sensor.x + width as i32 + 1;
                // Optional parameter to restrict x search range
                return match &x_range {
                    Some(range) => Some(start.max(range.start)..end.min(range.end)),
                    None => Some(start..end),
                };
            }

            None
        })
        .collect();

    ranges.sort_by_key(|range| range.start);

    // Here we are combining overlapping ranges to leave only distinct ranges.
    let mut merged_ranges: Vec<Range<i32>> = Vec::new();
    for range in ranges.iter() {
        match merged_ranges.last_mut() {
            Some(prev) => {
                if range.start > prev.end {
                    merged_ranges.push(range.clone());
                } else {
                    *prev = prev.start..prev.end.max(range.end);
                }
            }
            None => merged_ranges.push(range.clone()),
        }
    }

    // Count the number of total open cells by summing the lengths of the ranges and subtracting
    // the amount of cells that are known to be occupied by beacons
    let empty_cells = merged_ranges.iter().fold(0, |acc, range| {
        let beacons = beacons
            .iter()
            .filter(|beacon| beacon.y == y && range.contains(&beacon.x))
            // We use a HashSet to prevent duplicate beacons from appearing in the result
            .collect::<HashSet<_>>()
            .len();

        acc + range.len() as u32 - beacons as u32
    });

    (empty_cells, merged_ranges)
}

fn part_one(sensors: &Vec<Location>, beacons: &Vec<Location>) -> u32 {
    search_beacon(sensors, beacons, 2000000, None).0
}

fn part_two(sensors: &Vec<Location>, beacons: &Vec<Location>) -> u64 {
    for y in 0..=4000000 {
        let (_, ranges) = search_beacon(sensors, beacons, y, Some(0..4000001));
        // If we get two distinct, non-overlapping ranges,
        // the only open x coordinate must be at the end of the first range
        if ranges.len() == 2 {
            let x = ranges[0].end;
            return x as u64 * 4000000 + y as u64;
        }
    }

    0
}
