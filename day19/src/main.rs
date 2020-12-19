use std::collections::HashMap;
use std::fs;

fn part_1(rules: &HashMap<usize, Vec<String>>, messages: &str) -> usize {
    let rule_0 = rules.get(&0).unwrap();
    messages
        .lines()
        .filter(|&message| rule_0.contains(&message.to_owned()))
        .count()
}

fn part_2(rules: &HashMap<usize, Vec<String>>, messages: &str) -> usize {
    let rule_42 = rules.get(&42).unwrap();
    let rule_31 = rules.get(&31).unwrap();

    let chunk_size = rule_42.first().unwrap().len();

    messages
        .lines()
        .filter(|&msg| {
            if msg.len() % chunk_size == 0 {
                let no_of_chunks = msg.len() / chunk_size;

                let mut nr_of_42 = 0;
                let mut nr_of_31 = 0;
                let mut end_of_31 = false;
                for i in 1..=no_of_chunks {
                    let chunk =
                        &msg[msg.len() - chunk_size * i..(msg.len() - chunk_size * i) + chunk_size];
                    if !end_of_31 {
                        if rule_31.contains(&chunk.to_owned()) {
                            nr_of_31 += 1;
                        } else {
                            end_of_31 = true;
                        }
                    }
                    if end_of_31 {
                        if rule_42.contains(&chunk.to_owned()) {
                            nr_of_42 += 1;
                        } else {
                            return false;
                        }
                    }
                }

                return nr_of_31 > 0 && nr_of_42 > 0 && nr_of_42 - nr_of_31 >= 1;
            }
            false
        })
        .count()
}

fn main() {
    let content = fs::read_to_string("rules").unwrap();

    let mut known_rules = HashMap::new();
    let mut unknown_rules = HashMap::new();

    for line in content.lines() {
        let mut parts = line.split(": ");
        let rule_number = parts.next().unwrap().parse::<usize>().unwrap();
        let rule = parts.next().unwrap();

        if rule.contains("\"") {
            known_rules.insert(rule_number, vec![rule.trim_matches('"').to_owned()]);
        } else {
            let rule_definition = rule
                .split(" | ")
                .map(|part| {
                    part.split_ascii_whitespace()
                        .map(|rn| rn.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            unknown_rules.insert(rule_number, rule_definition);
        }
    }

    while !unknown_rules.is_empty() {
        let mut rules_to_transform = Vec::new();
        for (rule_number, rule_definition) in unknown_rules.iter() {
            if rule_definition
                .iter()
                .all(|or_part| or_part.iter().all(|rn| known_rules.contains_key(rn)))
            {
                rules_to_transform.push(*rule_number);
            }
        }

        for rule in rules_to_transform {
            let r = unknown_rules.get(&rule).unwrap();
            let nn = r
                .iter()
                .map(|or_part| {
                    let mut new_rule = Vec::new();

                    // I know, that is laziness on my part
                    if or_part.len() == 1 {
                        new_rule = known_rules.get(&or_part[0]).unwrap().clone();
                    } else if or_part.len() == 2 {
                        let first = known_rules.get(&or_part[0]).unwrap();
                        let second = known_rules.get(&or_part[1]).unwrap();

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
            known_rules.insert(rule, nn);
            unknown_rules.remove(&rule);
        }
    }

    let messages = fs::read_to_string("messages").unwrap();

    assert_eq!(part_1(&known_rules, &messages), 250);
    assert_eq!(part_2(&known_rules, &messages), 359);
}
