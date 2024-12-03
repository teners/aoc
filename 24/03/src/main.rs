fn solve_part_1(input: &String) -> i32 {
    use regex::Regex;
    let pattern = Regex::new(r"mul\((?P<first>\d+),(?P<second>\d+)\)").unwrap();

    let mut sum = 0i32;

    for regex_match in pattern.captures_iter(&input) {
        let first: i32 = regex_match.name("first").unwrap().as_str().parse().unwrap();
        let second: i32 = regex_match.name("second").unwrap().as_str().parse().unwrap();

        sum += first * second;
    }

    sum
}

/// Remove substrings between "don't()" and "do()"
/// and everything after the last unclosed "don't()".
fn clear_input(input: &String) -> String {
    use regex::Regex;
    let pattern = Regex::new(r"don't\(\).*?do\(\)|(don't\(\).*$)").unwrap();

    let single_line = input.replace("\n", "");
    pattern.replace_all(&single_line, "").into()
}

fn solve_part_2(input: &String) -> i32 {
    solve_part_1(&clear_input(&input))
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
    fn test_solve_part_1() {
        let input = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(161, solve_part_1(&input));
    }

    #[test]
    fn test_solve_part_2() {
        let input = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(48, solve_part_2(&input));
    }

    #[test]
    fn test_clear_input_remove_between_boundaries() {
        let input = String::from("expected_beforedon't()not_expectedo()-expected_after");
        let expected = String::from("expected_before-expected_after");

        assert_eq!(expected, clear_input(&input));
    }

    #[test]
    fn test_clear_input_trim_end() {
        let input = String::from("expecteddon't()not_expected");
        let expected = String::from("expected");

        assert_eq!(expected, clear_input(&input));
    }
}
