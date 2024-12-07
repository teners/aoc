struct Vector(i32, i32);

impl Vector {
    fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

mod directions {
    use super::Vector;

    pub const NORTH: Vector = Vector(-1, 0);
    pub const SOUTH: Vector = Vector(1, 0);
    pub const EAST: Vector = Vector(0, 1);
    pub const WEST: Vector = Vector(0, -1);
    pub const NORTHEAST: Vector = Vector(-1, 1);
    pub const NORTHWEST: Vector = Vector(-1, -1);
    pub const SOUTHEAST: Vector = Vector(1, 1);
    pub const SOUTHWEST: Vector = Vector(1, -1);

    #[rustfmt::skip]
    pub const ALL: [Vector; 8] = [
        NORTHWEST, NORTH, NORTHEAST,
        WEST,                  EAST,
        SOUTHWEST, SOUTH, SOUTHEAST,
    ];
}

struct Grid {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn at(&self, row: i32, col: i32) -> Option<char> {
        if row < 0 || col < 0 {
            return None;
        }
        if row as usize >= self.height {
            None
        } else if col as usize >= self.width {
            None
        } else {
            Some(self.grid[row as usize][col as usize])
        }
    }

    fn at_vector(&self, vector: &Vector) -> Option<char> {
        self.at(vector.0, vector.1)
    }

    fn find_xmas(&self, row: usize, col: usize, direction: &Vector) -> bool {
        let mut _row: i32 = row as i32;
        let mut _col: i32 = col as i32;
        for expected_char in "XMAS".chars() {
            match self.at(_row, _col) {
                Some(next_char) => {
                    if next_char != expected_char {
                        return false;
                    }
                    _row += direction.0;
                    _col += direction.1;
                }
                None => return false,
            }
        }
        return true;
    }

    /// Count "XMAS" occurrences in every direction.
    pub fn count_xmas(&self) -> u32 {
        let mut count = 0u32;

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                for direction in &directions::ALL {
                    if self.find_xmas(row, col, &direction) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn find_x_mas(&self, row: i32, col: i32) -> bool {
        if self.at(row, col).unwrap() != 'A' {
            return false;
        }

        let current_position = Vector(row, col);
        let northwest = self.at_vector(&current_position.add(&directions::NORTHWEST));
        let northeast = self.at_vector(&current_position.add(&directions::NORTHEAST));
        let southwest = self.at_vector(&current_position.add(&directions::SOUTHWEST));
        let southeast = self.at_vector(&current_position.add(&directions::SOUTHEAST));

        match (northwest, southeast, northeast, southwest) {
            (Some('M'), Some('S'), Some('M'), Some('S'))
            | (Some('S'), Some('M'), Some('S'), Some('M'))
            | (Some('M'), Some('S'), Some('S'), Some('M'))
            | (Some('S'), Some('M'), Some('M'), Some('S')) => true,
            _ => false,
        }
    }

    /// Count "MAS" occurrences in X shape.
    /// Example:
    /// M.S
    /// .A.
    /// M.S
    pub fn count_x_mas(&self) -> u32 {
        let mut count = 0u32;

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.find_x_mas(row as i32, col as i32) {
                    count += 1;
                }
            }
        }

        count
    }
}

impl From<String> for Grid {
    fn from(string: String) -> Self {
        let grid: Vec<Vec<char>> = string.lines().map(|line| line.chars().collect()).collect();
        let height = grid.len();
        let width = if height == 0 { 0 } else { grid[0].len() };
        Grid {
            grid,
            height,
            width,
        }
    }
}

fn solve_part_1(input: &String) -> u32 {
    let grid = Grid::from(input.clone());
    return grid.count_xmas();
}

fn solve_part_2(input: &String) -> u32 {
    let grid = Grid::from(input.clone());
    return grid.count_x_mas();
}

fn main() {
    let input: String = std::fs::read_to_string("data/input.txt").unwrap();

    println!("part 1: {}", solve_part_1(&input));
    println!("part 2: {}", solve_part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = String::from(
            "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX\n",
        );

        assert_eq!(18, solve_part_1(&input));
    }

    #[test]
    fn test_solve_part_2() {
        let input = String::from(
            "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX\n",
        );

        assert_eq!(9, solve_part_2(&input));
    }

    #[test]
    fn test_from() {
        let input = String::from(
            "\
            MMMSXXMASM\n\
            MSAMXMSMSA",
        );
        let expected: Vec<Vec<char>> = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        ];
        assert_eq!(expected, Grid::from(input).grid);
    }
}
