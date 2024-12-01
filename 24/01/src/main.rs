fn solve_part_1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for i in 0..left.len() {
        total += (left[i] - right[i]).abs();
    }
    return total;
}

fn solve_part_2(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut counter: HashMap<i32, i32> = HashMap::new();
    right.iter().for_each(|value| *counter.entry(*value).or_default() += 1);

    let mut total = 0;
    for value in left {
        total += value * *counter.entry(*value).or_default();
    }

    return total;
}

fn parse_line(line: &str) -> (i32, i32) {
    use std::str::FromStr;

    let parts = line
        .split("   ")
        .map(|part| i32::from_str(part).unwrap())
        .collect::<Vec<_>>();
    return (parts[0], parts[1]);
}

fn main() {
    let (mut left, mut right): (Vec<_>, Vec<_>) = std::fs::read_to_string("data/input.txt")
        .unwrap()
        .lines()
        .map(parse_line)
        .unzip();
    left.sort();
    right.sort();

    println!("part 1: {}", solve_part_1(&left, &right));
    println!("part 2: {}", solve_part_2(&left, &right));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part_1() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        left.sort();
        right.sort();
        assert_eq!(11, solve_part_1(&left, &right));
    }

    #[test]
    fn test_solve_part_2() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(31, solve_part_2(&left, &right));
    }

    #[test]
    fn test_parse() {
        let line = "10   12";
        assert_eq!((10, 12), parse_line(line));
    }
}
