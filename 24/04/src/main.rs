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
        let _row: usize = row.try_into().unwrap();
        let _col: usize = col.try_into().unwrap();
        if _row >= self.height {
            None
        } else if _col >= self.width {
            None
        } else {
            Some(self.grid[_row][_col])
        }
    }

    fn find_xmas(&self, row: usize, col: usize, direction: &(i32, i32)) -> bool {
        let mut _row: i32 = row.try_into().unwrap();
        let mut _col: i32 = col.try_into().unwrap();
        for expected_char in "XMAS".chars() {
            match self.at(_row, _col) {
                Some(next_char) => {
                    if next_char != expected_char {
                        return false;
                    }
                    _row += direction.0;
                    _col += direction.1;
                },
                None => return false,
            }
        }
        return true;
    }

    pub fn count_xmas(&self) -> u32 {
        let mut count = 0u32;

        let directions = vec![
            (0, 1),   // horizontal
            (0, -1),  // horizontal reverse
            (1, 0),   // vertical
            (-1, 0),  // vertical reverse
            (1, 1),   // diagonal
            (1, -1),  // diagonal reverse
            (-1, 1),  // reverse diagonal
            (-1, -1), // reverse diagonal reverse
        ];

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                for direction in &directions {
                    if self.find_xmas(row, col, &direction) {
                        count += 1;
                    }
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

fn solve_part_2() -> () {}

fn main() {
    let input: String = std::fs::read_to_string("data/input.txt").unwrap();

     println!("part 1: {}", solve_part_1(&input));
    // println!("part 2: {}", solve_part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let input = String::from("\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX\n"
        );

        assert_eq!(18, solve_part_1(&input));
    }

    #[test]
    fn test_solve_part_2() {}

    #[test]
    fn test_parse() {
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
