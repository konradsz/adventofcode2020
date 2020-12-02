use regex::Regex;
use std::fs;

#[macro_use]
extern crate lazy_static;

struct Entry {
    value_1: usize,
    value_2: usize,
    letter: char,
    password: String,
}

fn parse_entry(line: &str) -> Option<Entry> {
    lazy_static! {
        static ref ENTRY: Regex =
            Regex::new(r"(?P<value_1>\d+)-(?P<value_2>\d+) (?P<letter>\S{1}): (?P<password>\S+)$")
                .unwrap();
    }

    if ENTRY.is_match(line) {
        let caps = ENTRY.captures(line).unwrap();
        Some(Entry {
            value_1: caps["value_1"].parse::<usize>().unwrap(),
            value_2: caps["value_2"].parse::<usize>().unwrap(),
            letter: caps["letter"].chars().next().unwrap(),
            password: String::from(&caps["password"]),
        })
    } else {
        None
    }
}

fn part_1(entries: &[Entry]) -> usize {
    let policy = |e: &&Entry| {
        let count = e.password.chars().filter(|&c| c == e.letter).count();
        count >= e.value_1 && count <= e.value_2
    };

    entries.iter().filter(policy).count()
}

fn part_2(entries: &[Entry]) -> usize {
    let policy = |e: &&Entry| {
        (e.password.chars().nth(e.value_1 - 1).unwrap() == e.letter)
            ^ (e.password.chars().nth(e.value_2 - 1).unwrap() == e.letter)
    };

    entries.iter().filter(policy).count()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let entries: Vec<Entry> = content.lines().filter_map(parse_entry).collect();

    assert_eq!(519, part_1(&entries));
    assert_eq!(708, part_2(&entries));
}
