use regex::Regex;
use std::fs;

#[derive(Clone)]
struct Cave {
    pub map: Vec<Vec<bool>>,
}

impl Cave {
    fn new() -> Self {
        Self { map: vec![vec![]] }
    }

    /// Sets the width and height of the current map.
    /// Does not truncate if the width/height is less than the current dimensions.
    fn resize(&mut self, width: usize, height: usize) {
        if self.map.len() < height {
            self.map.resize(height, vec![false; self.map[0].len()]);
        }
        if self.map[0].len() < width {
            for row in self.map.iter_mut() {
                row.resize(width, false);
            }
        }
    }

    fn draw_wall(&mut self, start: (usize, usize), end: (usize, usize)) {
        let (from_x, from_y) = start;
        let (to_x, to_y) = end;

        // Adding 2 to the height gives us room for the floor
        // Adding 2 to the width gives us room for particles that could go out of bounds to the right
        self.resize(from_x.max(to_x) + 2, from_y.max(to_y) + 2);

        // Horizontal Path
        if from_y == to_y {
            for x in from_x.min(to_x)..from_x.max(to_x) + 1 {
                self.map[from_y][x] = true;
            }
        }
        // Vertical Path
        if from_x == to_x {
            for y in from_y.min(to_y)..from_y.max(to_y) + 1 {
                self.map[y][from_x] = true;
            }
        }
    }
}

fn main() {
    let raw = fs::read_to_string("./inputs/day14.txt").unwrap();

    let re = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();

    let mut cave = Cave::new();

    raw.lines().for_each(|line| {
        // Iterate over every rock path
        re.captures_iter(line)
            .map(|loc| {
                (
                    loc["x"].parse::<usize>().unwrap(),
                    loc["y"].parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<_>>()
            // Iterate over every pair of coordinates to draw walls
            .windows(2)
            .for_each(|pair| cave.draw_wall(pair[0], pair[1]));
    });

    println!("Part 1: {}", part_one(&cave));
    println!("Part 2: {}", part_two(&cave));
}

fn simulate_sand(cave: &mut Cave, solid_floor: bool) -> usize {
    let mut count = 0;

    'outer: loop {
        let (mut x, mut y) = (500, 0);

        // We end simulation if the source location is solid
        if cave.map[y][x] {
            break 'outer;
        }

        'inner: loop {
            if y + 1 >= cave.map.len() {
                // If floor is simulated, we place a particle
                // at the bottom and continue the simulation
                if solid_floor {
                    cave.map[y][x] = true;
                    count += 1;
                    break 'inner;
                }
                // If the floor is not simulated, we end the simulation
                // when a particle goes out of bounds (into the abyss)
                break 'outer;
            }

            if !cave.map[y + 1][x] {
                // Location below is not solid
                y += 1;
            } else if !cave.map[y + 1][x - 1] {
                // Location below and to the left below is not solid
                y += 1;
                x -= 1;
            } else if !cave.map[y + 1][x + 1] {
                // Location below and to the right below is not solid
                y += 1;
                x += 1;
                // We need extra space if a particle goes out of bounds to the right
                cave.resize(x + 2, cave.map.len());
            } else {
                // Nowhere else to go, sand particle settles and current location is set to solid
                cave.map[y][x] = true;
                count += 1;
                break 'inner;
            }
        }
    }

    count
}

fn part_one(cave: &Cave) -> usize {
    simulate_sand(&mut cave.clone(), false)
}

fn part_two(cave: &Cave) -> usize {
    simulate_sand(&mut cave.clone(), true)
}
