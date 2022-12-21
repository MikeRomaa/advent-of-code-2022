pub struct HeightMap {
    pub heights: Vec<Vec<u8>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl HeightMap {
    pub fn from(raw: String) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);

        let heights: Vec<Vec<u8>> = raw
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        'S' => {
                            start = (x, y);
                            0
                        }
                        'E' => {
                            end = (x, y);
                            25
                        }
                        _ => (char as u8) - 97,
                    })
                    .collect()
            })
            .collect();

        Self {
            heights,
            start,
            end,
        }
    }

    pub fn get_height(&self, loc: (usize, usize)) -> u8 {
        self.heights[loc.1][loc.0]
    }

    pub fn get_neighbors(&self, loc: (usize, usize), reverse: bool) -> Vec<(usize, usize)> {
        let mut candidates = Vec::new();

        // West
        if loc.0 > 0 {
            candidates.push((loc.0 - 1, loc.1));
        }
        // East
        if loc.0 + 1 < self.heights[0].len() {
            candidates.push((loc.0 + 1, loc.1));
        }
        // North
        if loc.1 > 0 {
            candidates.push((loc.0, loc.1 - 1));
        }
        // South
        if loc.1 + 1 < self.heights.len() {
            candidates.push((loc.0, loc.1 + 1));
        }

        // Check if new location is traversable (no more than 1 unit higher)
        candidates
            .into_iter()
            .filter(|&new_loc| {
                if reverse {
                    // Returns neighbors that we can come from to the current location
                    self.get_height(loc) as i8 - self.get_height(new_loc) as i8 <= 1
                } else {
                    // Returns neighbors that we can go to from the current location
                    self.get_height(new_loc) as i8 - self.get_height(loc) as i8 <= 1
                }
            })
            .collect()
    }
}
