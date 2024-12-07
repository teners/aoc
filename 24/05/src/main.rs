type RuleSet = std::collections::HashSet<u32>;
type Rules = std::collections::HashMap<u32, RuleSet>;
type Update = Vec<u32>;

fn solve_part_1(rules: &Rules, updates: &Vec<Update>) -> u32 {
    let mut count = 0u32;

    for update in updates {
        let mut seen: RuleSet = std::collections::HashSet::new();
        seen.insert(update[update.len() - 1]);
        for i in (0..update.len() - 1).rev() {
            let value = update[i];
            let ruleset = rules.get(&value);
            if ruleset.is_none() {
                break;
            }
            if seen.is_subset(&ruleset.unwrap()) {
                seen.insert(value);
            } else {
                break;
            }
        }
        if seen.len() == update.len() {
            let middle = update.len() / 2;
            count += update[middle] as u32
        }
    }

    count
}

fn solve_part_2(rules: &Rules, updates: &Vec<Update>) -> u32 {
    use std::cmp::Ordering;

    let mut count = 0u32;

    for update in updates {
        let mut seen: RuleSet = std::collections::HashSet::new();
        seen.insert(update[update.len() - 1]);
        for i in (0..update.len() - 1).rev() {
            let value = update[i];
            let ruleset = rules.get(&value);
            if ruleset.is_none() {
                break;
            }
            if seen.is_subset(&ruleset.unwrap()) {
                seen.insert(value);
            } else {
                break;
            }
        }
        if seen.len() != update.len() {
            let mut sorted_update = update.clone();
            sorted_update.sort_by(|l, r| {
                let l_ruleset = rules.get(&l);
                if l_ruleset.is_some_and(|ruleset| ruleset.contains(r)) {
                    return Ordering::Less;
                }
                let r_ruleset = rules.get(&r);
                if r_ruleset.is_some_and(|ruleset| ruleset.contains(l)) {
                    return Ordering::Greater;
                }
                return Ordering::Equal;
            });
            let middle = sorted_update.len() / 2;
            count += sorted_update[middle] as u32
        }
    }

    count
}

fn parse(input: &String) -> (Rules, Vec<Update>) {
    use std::str::FromStr;

    const DELIMITER: &str = "\n\n";
    let (unparsed_rules, unparsed_updates) = input.split_once(DELIMITER).unwrap();

    let mut rules = Rules::new();
    for line in unparsed_rules.lines() {
        let (unparsed_left, unparsed_right) = line.split_once("|").unwrap();
        let left = u32::from_str(unparsed_left).unwrap();
        let right = u32::from_str(unparsed_right).unwrap();
        rules
            .entry(left)
            .and_modify(|rules_for_key: &mut RuleSet| {
                rules_for_key.insert(right);
            })
            .or_default()
            .insert(right);
    }

    let mut updates = Vec::new();
    for line in unparsed_updates.lines() {
        updates.push(line.split(",").map(|v| u32::from_str(v).unwrap()).collect());
    }

    (rules, updates)
}

fn main() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();

    let (rules, updates) = parse(&input);
    println!("part 1: {}", solve_part_1(&rules, &updates));
    println!("part 2: {}", solve_part_2(&rules, &updates));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47\
    ";

    #[test]
    fn test_solve_part_1() {
        let input = String::from(INPUT);
        let (rules, updates) = parse(&input);
        assert_eq!(143, solve_part_1(&rules, &updates));
    }

    #[test]
    fn test_solve_part_2() {
        let input = String::from(INPUT);
        let (rules, updates) = parse(&input);
        assert_eq!(123, solve_part_2(&rules, &updates));
    }

    #[test]
    fn test_parse() {
        use std::collections::{HashMap, HashSet};

        let input = String::from(
            "\
            47|53\n\
            97|13\n\
            97|61\n\
            \n\
            75,47,61,53,29\n\
            97,61,53,29,13\
            ",
        );
        let mut expected_rules = HashMap::new();
        let mut ruleset_47 = HashSet::new();
        ruleset_47.insert(53);
        let mut ruleset_97 = HashSet::new();
        ruleset_97.insert(13);
        ruleset_97.insert(61);
        expected_rules.insert(47, ruleset_47);
        expected_rules.insert(97, ruleset_97);

        let expected_updates = vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13]];

        let (rules, updates) = parse(&input);

        assert_eq!(expected_rules, rules);
        assert_eq!(expected_updates, updates);
    }
}
