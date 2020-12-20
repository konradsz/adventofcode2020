use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Hash)]
struct Border<'a>(&'a str);
struct Tile<'a>([&'a Border<'a>; 4]);

struct Orientation {
    top: u64,
    right: u64,
    bottom: u64,
    left: u64,
}

fn generate_orientations(tile: &Tile) -> Vec<Orientation> {
    let calculate_hash = |border: &Border| {
        let mut hasher = DefaultHasher::new();
        border.hash(&mut hasher);
        hasher.finish()
    };

    let generate_rotations = |tile: &Tile| -> Vec<Orientation> {
        (0..4)
            .map(|i| Orientation {
                top: calculate_hash(&tile.0[i % 4]),
                right: calculate_hash(&tile.0[(i + 1) % 4]),
                bottom: calculate_hash(&tile.0[(i + 2) % 4]),
                left: calculate_hash(&tile.0[(i + 3) % 4]),
            })
            .collect()
    };

    let mut orientations = Vec::new();
    orientations.append(&mut generate_rotations(tile));

    // flip horizontally
    let right = tile.0[1].0.chars().rev().collect::<String>();
    let right = Border(&right);
    let left = tile.0[3].0.chars().rev().collect::<String>();
    let left = Border(&left);
    orientations.append(&mut generate_rotations(&Tile([
        tile.0[2], &right, tile.0[0], &left,
    ])));

    // flip vertically
    let top = tile.0[0].0.chars().rev().collect::<String>();
    let top = Border(&top);
    let bottom = tile.0[2].0.chars().rev().collect::<String>();
    let bottom = Border(&bottom);
    orientations.append(&mut generate_rotations(&Tile([
        &top, tile.0[3], &bottom, tile.0[1],
    ])));

    orientations
}

fn part_1(tile_orientations: &HashMap<u64, Vec<Orientation>>) -> u64 {
    let mut result = HashSet::new();
    for (i, tile_orientation_primary) in tile_orientations.iter() {
        'outer: for corner_candidate in tile_orientation_primary.iter() {
            for (j, tile_orientation_secondary) in tile_orientations.iter() {
                if i == j {
                    continue;
                }

                if tile_orientation_secondary
                    .iter()
                    .any(|neighbour_candidate| {
                        neighbour_candidate.bottom == corner_candidate.top
                            || neighbour_candidate.right == corner_candidate.left
                    })
                {
                    continue 'outer;
                }
            }
            result.insert(*i);
        }
    }

    result.iter().product()
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let mut orientations = HashMap::new();
    let mut tile_str: Vec<&str> = Vec::new();
    let mut tile_id = 0;
    for line in content.lines() {
        if line.contains("Tile") {
            tile_id = line
                .trim_start_matches("Tile ")
                .trim_end_matches(':')
                .parse::<u64>()
                .unwrap();
        } else if line.is_empty() {
            tile_str.clear();
        } else {
            tile_str.push(line);
        }

        if tile_str.len() == 10 {
            let right = tile_str
                .iter()
                .map(|&line| line.chars().last().unwrap())
                .collect::<String>();
            let left = tile_str
                .iter()
                .map(|&line| line.chars().next().unwrap())
                .collect::<String>();
            orientations.insert(
                tile_id,
                generate_orientations(&Tile([
                    &Border(tile_str[0]),
                    &Border(&right),
                    &Border(tile_str[9]),
                    &Border(&left),
                ])),
            );
        }
    }

    assert_eq!(part_1(&orientations), 108_603_771_107_737);
}
