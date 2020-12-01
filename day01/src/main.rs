use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_1(entries: &HashSet<usize>) -> usize {
    for first in entries.iter() {
        let needed = 2020 - first;
        if let Some(second) = entries.get(&needed) {
            return first * second;
        }
    }
    unreachable!()
}

fn part_2(entries: &HashSet<usize>) -> usize {
    for first in entries.iter() {
        for second in entries.iter() {
            if let Some(needed) = 2020usize.checked_sub(first + second) {
                if let Some(third) = entries.get(&needed) {
                    return first * second * third;
                }
            }
        }
    }
    unreachable!()
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let entries: HashSet<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    assert_eq!(part_1(&entries), 658_899);
    assert_eq!(part_2(&entries), 155_806_250);

    Ok(())
}
