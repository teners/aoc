type Grid = Vec<Vec<char>>;

fn find_horizontal(matrix: &Grid, current_row: usize, current_column: usize) -> bool {
    if current_column + 3 >= matrix[current_row].len() {
        return false;
    }
    if matrix[current_row][current_column] == 'X'
        && matrix[current_row][current_column + 1] == 'M'
        && matrix[current_row][current_column + 2] == 'A'
        && matrix[current_row][current_column + 3] == 'S' {
        return true;
    }
    if matrix[current_row][current_column] == 'S'
        && matrix[current_row][current_column + 1] == 'A'
        && matrix[current_row][current_column + 2] == 'M'
        && matrix[current_row][current_column + 3] == 'X' {
        return true;
    }
    return false;
}

fn find_vertical(matrix: &Grid, current_row: usize, current_column: usize) -> bool {
    if current_row + 3 >= matrix[current_row].len() {
        return false;
    }
    if matrix[current_row][current_column] == 'X'
        && matrix[current_row + 1][current_column] == 'M'
        && matrix[current_row + 2][current_column] == 'A'
        && matrix[current_row + 3][current_column] == 'S' {
        return true;
    }
    if matrix[current_row][current_column] == 'S'
        && matrix[current_row + 1][current_column] == 'A'
        && matrix[current_row + 2][current_column] == 'M'
        && matrix[current_row + 3][current_column] == 'X' {
        return true;
    }
    return false;
}

fn find_diagonal(matrix: &Grid, current_row: usize, current_column: usize) -> bool {
    if current_column + 3 >= matrix[current_row].len() || current_row + 3 >= matrix.len() {
        return false;
    }
    if matrix[current_row][current_column] == 'X'
        && matrix[current_row + 1][current_column + 1] == 'M'
        && matrix[current_row + 2][current_column + 2] == 'A'
        && matrix[current_row + 3][current_column + 3] == 'S' {
        return true;
    }
    if matrix[current_row][current_column] == 'S'
        && matrix[current_row + 1][current_column + 1] == 'A'
        && matrix[current_row + 2][current_column + 2] == 'M'
        && matrix[current_row + 3][current_column + 3] == 'X' {
        return true;
    }
    return false;
}

fn find_diagonal_reverse(matrix: &Grid, current_row: usize, current_column: usize) -> bool {
    if current_column < 3 || current_row + 3 >= matrix.len() {
        return false;
    }
    if matrix[current_row][current_column] == 'X'
        && matrix[current_row + 1][current_column - 1] == 'M'
        && matrix[current_row + 2][current_column - 2] == 'A'
        && matrix[current_row + 3][current_column - 3] == 'S' {
        return true;
    }
    if matrix[current_row][current_column] == 'S'
        && matrix[current_row + 1][current_column - 1] == 'A'
        && matrix[current_row + 2][current_column - 2] == 'M'
        && matrix[current_row + 3][current_column - 3] == 'X' {
        return true;
    }
    return false;
}

fn solve_part_1(input: &Grid) -> i32 {
    let mut total = 0i32;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if find_horizontal(&input, i, j) {
                total += 1;
            }
            if find_vertical(&input, i, j) {
                total += 1;
            }
            if find_diagonal(&input, i, j) {
                total += 1;
            }
            if find_diagonal_reverse(&input, i, j) {
                total += 1;
            }
        }
    }

    total
}

fn solve_part_2() -> () {}

fn parse(input: String) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn main() {
    let input: String = std::fs::read_to_string("data/input.txt").unwrap();
    let matrix = parse(input);

     println!("part 1: {}", solve_part_1(&matrix));
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

        assert_eq!(18, solve_part_1(&parse(input)));
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
        let expected: Grid = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        ];
        assert_eq!(expected, parse(input));
    }
}
