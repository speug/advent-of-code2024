use itertools::Itertools;
use std::fmt;

advent_of_code::solution!(5);

struct Rule {
    first: u8,
    second: u8,
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rule({}, {})", self.first, self.second)
    }
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Vec<u8>>) {
    let mut rules = Vec::new();
    let mut manuals = Vec::new();
    for line in input.lines() {
        if line.contains("|") {
            let parts: Vec<u8> = line
                .split("|")
                .into_iter()
                .map(|v| v.parse::<u8>().unwrap())
                .collect();
            if parts.len() == 2 {
                let rule = Rule {
                    first: parts[0],
                    second: parts[1],
                };
                rules.push(rule);
            } else {
                panic!("Line {:?} could not be parsed!", line)
            }
        } else if line.contains(",") {
            let parts: Vec<u8> = line
                .split(",")
                .into_iter()
                .map(|v| v.parse::<u8>().unwrap())
                .collect();
            manuals.push(parts);
        }
    }
    let mut manuals_with_dups = Vec::new();
    for (i, pages) in manuals.clone().into_iter().enumerate() {
        if pages.len() > Vec::from_iter(pages.into_iter().unique()).len() {
            manuals_with_dups.push(i);
        }
    }
    if manuals_with_dups.len() == 0 {
        println!("No duplicate pages in any manuals!");
    } else {
        println!(
            "Following manual indices have duplicates: {:?}",
            manuals_with_dups
        );
    }
    (rules, manuals)
}

fn check_rule(pages: &Vec<u8>, rule: &Rule) -> bool {
    if !(pages.contains(&rule.first) && pages.contains(&rule.second)) {
        return true;
    }
    let pos_first = pages.iter().position(|&v| v == rule.first);
    let pos_second = pages.iter().position(|&v| v == rule.second);
    if pos_first > pos_second {
        return false;
    }
    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, manuals) = parse_input(input);
    let mut out = 0u64;
    for manual in manuals {
        let mut rules_ok = true;
        for rule in &rules {
            if !check_rule(&manual, &rule) {
                rules_ok = false;
                break;
            }
        }
        if rules_ok {
            out += manual[manual.len() / 2] as u64;
        }
    }
    Some(out)
}

fn sort_by_rules(pages: &mut Vec<u8>, rules: &Vec<Rule>) {
    for rule in rules {
        if !(pages.contains(&rule.first) && pages.contains(&rule.second)) {
            continue;
        }
        let pos_first = pages.iter().position(|&v| v == rule.first);
        let pos_second = pages.iter().position(|&v| v == rule.second);
        if pos_first > pos_second {
            pages.swap(pos_first.unwrap(), pos_second.unwrap());
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, manuals) = parse_input(input);
    let mut out = 0u64;
    for mut manual in manuals {
        for rule in &rules {
            if !check_rule(&manual, &rule) {
                sort_by_rules(&mut manual, &rules);
                println!("{:?}", manual);
                out += manual[manual.len() / 2] as u64;
                break;
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
