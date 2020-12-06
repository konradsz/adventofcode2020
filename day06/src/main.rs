use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn part_1(groups: &[&str]) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut answers = HashSet::new();
            for person in group.lines() {
                person.chars().for_each(|c| {
                    answers.insert(c);
                });
            }
            answers.len()
        })
        .sum()
}

fn part_2(groups: &[&str]) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut answers = HashMap::new();
            for person in group.lines() {
                person.chars().for_each(|c| {
                    let counter = answers.entry(c).or_insert(0);
                    *counter += 1;
                });
            }

            answers
                .values()
                .filter(|&&a| a == group.lines().count())
                .count()
        })
        .sum()
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let groups: Vec<&str> = content.split_terminator("\n\n").collect();

    assert_eq!(part_1(&groups), 6_542);
    assert_eq!(part_2(&groups), 3_299);
}
