use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let mut ingredients_intersections: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut ingredients_occurrences = HashMap::new();
    for line in content.lines() {
        let mut parts = line.split("(contains ");

        let mut ingredients = HashSet::new();
        for ingredient in parts.next().unwrap().split_ascii_whitespace() {
            ingredients.insert(ingredient);
            let entry = ingredients_occurrences.entry(ingredient).or_insert(0);
            *entry += 1;
        }

        let allergens = parts
            .next()
            .unwrap()
            .trim_end_matches(')')
            .split(',')
            .map(|a| a.trim())
            .collect::<Vec<_>>();

        for allergen in allergens {
            let entry = ingredients_intersections
                .entry(allergen)
                .or_insert(ingredients.clone());
            *entry = entry
                .intersection(&ingredients)
                .cloned()
                .collect::<HashSet<_>>();
        }
    }

    // after analysis of content of ingredients_intersections
    let allergic = [
        "frtfg", "hvnkk", "vvfjj", "qnvx", "cpxmpc", "cljf", "qsjszn", "qmrps",
    ];

    let part_1 = || {
        ingredients_occurrences
            .iter()
            .filter(|(k, _)| !allergic.contains(k))
            .map(|(_, &v)| v)
            .sum::<i32>()
    };

    assert_eq!(part_1(), 2307);

    /* PART 2:
    Sorted alphabetically by their allergen:
    "dairy": "cljf",
    "eggs": "frtfg",
    "fish": "vvfjj",
    "nuts": "qmrps"
    "peanuts": "hvnkk",
    "shellfish": "qnvx",
    "soy": "cpxmpc",
    "wheat": "qsjszn",

    what gives: cljf,frtfg,vvfjj,qmrps,hvnkk,qnvx,cpxmpc,qsjszn
    */
}
