fn parse_line(line: &str) -> Vec<i32> {
    use std::str::FromStr;
    return line
        .split(" ")
        .map(|part| i32::from_str(part).unwrap())
        .collect();
}

/// Report is safe if:
/// 1. Items are either increasing or decreasing consistently
/// 2. Any two adjacent items differ by at least 1 and at most 3
fn is_report_safe(report: Vec<i32>) -> bool {
    let is_order_consistent =
        report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| a > b);
    // dirty-ish way to emulate itertools::Itertools::tuple_windows using is_sorted_by
    let has_correct_deltas = report.is_sorted_by(|a, b| (1..=3).contains(&(a - b).abs()));
    return is_order_consistent && has_correct_deltas;
}

/// Report is safe if:
/// 1. Items are either increasing or decreasing consistently
/// 2. Any two adjacent items differ by at least 1 and at most 3
/// 3. A single item could be removed and the previous rules still apply
fn is_report_safe_with_tolerance(report: Vec<i32>) -> bool {
    if is_report_safe(report.clone()) {
        return true;
    };

    // brute force through permutations by removing one element at a time
    (0..report.len()).any(|idx| {
        let mut with_tolerance = report.clone();
        with_tolerance.remove(idx);
        is_report_safe(with_tolerance)
    })
}

fn solve_part_1(input: &String) -> i32 {
    let solution = input
        .lines()
        .map(parse_line)
        .map(is_report_safe)
        .fold(0, |acc, value| acc + i32::from(value));
    solution
}

fn solve_part_2(input: &String) -> i32 {
    let solution = input
        .lines()
        .map(parse_line)
        .map(is_report_safe_with_tolerance)
        .fold(0, |acc, value| acc + i32::from(value));
    solution
}

fn main() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();

    println!("part 1: {}", solve_part_1(&input));
    println!("part 2: {}", solve_part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "7 6 4 2 1";
        let expected = vec![7, 6, 4, 2, 1];
        assert_eq!(expected, parse_line(line));
    }

    #[test]
    fn test_is_report_safe_with_safe() {
        let report = vec![7, 6, 4, 2, 1];
        assert_eq!(true, is_report_safe(report));
    }

    #[test]
    fn test_is_report_safe_with_mixed_order() {
        let report = vec![7, 6, 4, 2, 4];
        assert_eq!(false, is_report_safe(report));
    }

    #[test]
    fn test_is_report_safe_with_low_delta() {
        let report = vec![7, 6, 4, 2, 2];
        assert_eq!(false, is_report_safe(report));
    }

    #[test]
    fn test_is_report_safe_with_high_delta() {
        let report = vec![10, 6, 4, 2, 1];
        assert_eq!(false, is_report_safe(report));
    }

    #[test]
    fn test_is_report_safe_with_tolerance_with_safe() {
        let report = vec![7, 6, 4, 2, 1];
        assert_eq!(true, is_report_safe_with_tolerance(report));
    }

    #[test]
    fn test_is_report_safe_with_tolerance_with_one_error() {
        let report = vec![7, 6, 4, 2, 4];
        assert_eq!(true, is_report_safe_with_tolerance(report));
    }

    #[test]
    fn test_is_report_safe_with_tolerance_with_two_errors() {
        let report = vec![6, 6, 4, 2, 4];
        assert_eq!(false, is_report_safe_with_tolerance(report));
    }

    #[test]
    fn test_solve_part_1() {
        let input = String::from(
            "\
            7 6 4 2 1\n\
            1 2 7 8 9\n\
            9 7 6 2 1\n\
            1 3 2 4 5\n\
            8 6 4 4 1\n\
            1 3 6 7 9",
        );
        assert_eq!(2, solve_part_1(&input));
    }

    #[test]
    fn test_solve_part_2() {
        let input = String::from(
            "\
            7 6 4 2 1\n\
            1 2 7 8 9\n\
            9 7 6 2 1\n\
            1 3 2 4 5\n\
            8 6 4 4 1\n\
            1 3 6 7 9",
        );
        assert_eq!(4, solve_part_2(&input));
    }
}
