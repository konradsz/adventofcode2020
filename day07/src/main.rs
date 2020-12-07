use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn collect_bags<'a>(
    found: &mut HashSet<&'a str>,
    current: &str,
    bags: &'a HashMap<String, Vec<(u32, String)>>,
) {
    bags.iter().for_each(|(k, v)| {
        if v.iter().any(|(_, other)| other == current) {
            found.insert(k);

            collect_bags(found, k, bags);
        }
    });
}

fn count_bags(
    total: &mut u32,
    current: &str,
    current_count: u32,
    bags: &HashMap<String, Vec<(u32, String)>>,
) {
    bags.get(current).unwrap().iter().for_each(|(n, name)| {
        *total += current_count * n;
        count_bags(total, name, current_count * *n, bags);
    });
}

fn part_1(bags: &HashMap<String, Vec<(u32, String)>>) -> usize {
    let mut found: HashSet<&str> = HashSet::new();
    collect_bags(&mut found, "shiny gold", &bags);
    found.len()
}

fn part_2(bags: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    let mut total = 0;
    count_bags(&mut total, "shiny gold", 1, &bags);
    total
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let mut bags: HashMap<String, Vec<(u32, String)>> = HashMap::new();
    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let name = format!("{} {}", parts.next().unwrap(), parts.next().unwrap());

        parts.next();
        parts.next();

        let mut inside = Vec::new();
        while let Some(next) = parts.next() {
            if let Ok(count) = next.parse::<u32>() {
                let bag = format!("{} {}", parts.next().unwrap(), parts.next().unwrap());
                inside.push((count, bag));
            }
        }

        bags.insert(name, inside);
    }

    assert_eq!(part_1(&bags), 101);
    assert_eq!(part_2(&bags), 108_636);
}
