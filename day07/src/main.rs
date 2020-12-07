use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn collect_bags(
    found: &mut HashSet<String>,
    current: &str,
    bags: &HashMap<String, Vec<(u32, String)>>,
) {
    for (k, v) in bags.iter() {
        if v.iter().any(|(_, other)| other == current) {
            found.insert(k.clone());

            collect_bags(found, k, bags);
        }
    }
}

fn count_bags(
    total: &mut u32,
    current: &str,
    current_count: u32,
    bags: &HashMap<String, Vec<(u32, String)>>,
) {
    let next = bags.get(current).unwrap();

    for (n, name) in next {
        *total += current_count * n;
        count_bags(total, name, current_count * *n, bags);
    }
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

    let mut found: HashSet<String> = HashSet::new();
    collect_bags(&mut found, "shiny gold", &bags);
    assert_eq!(found.len(), 101); // part_1

    let mut total = 0;
    count_bags(&mut total, "shiny gold", 1, &bags);
    assert_eq!(total, 108_636);
}
