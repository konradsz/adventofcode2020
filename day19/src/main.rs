use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("rules").unwrap();

    let mut uncertain_rules = HashMap::new();
    let mut certain_rules = HashMap::new();

    for line in content.lines() {
        let mut parts = line.split(": ");
        let rule_number = parts.next().unwrap().parse::<usize>().unwrap();
        let rule = parts.next().unwrap();

        if rule.contains("\"") {
            certain_rules.insert(rule_number, vec![rule.trim_matches('"').to_owned()]);
        } else {
            let rule_definition = rule
                .split(" | ")
                .map(|part| {
                    part.split_ascii_whitespace()
                        .map(|rn| rn.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            uncertain_rules.insert(rule_number, rule_definition);
        }
    }

    while !uncertain_rules.is_empty() {
        let mut rules_to_transform = Vec::new();
        for (rule_number, rule_definition) in uncertain_rules.iter() {
            if rule_definition
                .iter()
                .all(|or_part| or_part.iter().all(|rn| certain_rules.contains_key(rn)))
            {
                rules_to_transform.push(*rule_number);
            }
        }

        for rule in rules_to_transform {
            let r = uncertain_rules.get(&rule).unwrap();
            let nn = r
                .iter()
                .map(|or_part| {
                    let mut new_rule = Vec::new();

                    // I know, that is laziness on my part
                    if or_part.len() == 1 {
                        new_rule = certain_rules.get(&or_part[0]).unwrap().clone();
                    } else if or_part.len() == 2 {
                        let first = certain_rules.get(&or_part[0]).unwrap();
                        let second = certain_rules.get(&or_part[1]).unwrap();

                        for f in first.iter() {
                            for s in second.iter() {
                                new_rule.push(format!("{}{}", f, s));
                            }
                        }
                    }
                    new_rule
                })
                .flatten()
                .collect::<Vec<String>>();
            certain_rules.insert(rule, nn);
            uncertain_rules.remove(&rule);
        }
    }

    let messages = fs::read_to_string("messages").unwrap();

    let rule_0 = certain_rules.get(&0).unwrap();
    let matching_messages = messages
        .lines()
        .filter(|&message| rule_0.contains(&message.to_owned()))
        .count();

    assert_eq!(matching_messages, 250);
}
