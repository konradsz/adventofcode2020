use std::collections::HashMap;
use std::fs;

fn get_coordinates(line: &str) -> (i32, i32) {
    let mut coordinates = (0, 0);
    let mut chs = line.chars();
    while let Some(first) = chs.next() {
        match first {
            'e' => coordinates.0 += 1,
            'w' => coordinates.0 -= 1,
            's' => {
                let second = chs.next().unwrap();
                match (first, second) {
                    ('s', 'e') => {
                        coordinates.0 += 1;
                        coordinates.1 -= 1;
                    }
                    ('s', 'w') => coordinates.1 -= 1,
                    _ => panic!("unknown direction"),
                }
            }
            'n' => {
                let second = chs.next().unwrap();
                match (first, second) {
                    ('n', 'e') => coordinates.1 += 1,
                    ('n', 'w') => {
                        coordinates.0 -= 1;
                        coordinates.1 += 1;
                    }
                    _ => panic!("unknown direction"),
                }
            }
            _ => panic!("unknown direction"),
        }
    }

    coordinates
}

fn parse_input(input: &str) -> HashMap<(i32, i32), i32> {
    let mut tiles = HashMap::new();

    input
        .lines()
        .map(|line| get_coordinates(line))
        .for_each(|coords| {
            let entry = tiles.entry(coords).or_insert(0);
            *entry += 1;
        });

    tiles
}

fn part_1(tiles: &HashMap<(i32, i32), i32>) -> usize {
    tiles.values().filter(|&v| v % 2 != 0).count()
}

fn part_2(mut tiles: HashMap<(i32, i32), i32>) -> usize {
    for _ in 0..100 {
        let mut new_tiles = tiles.clone();
        tiles.keys().for_each(|&k| {
            let neighbours = [
                (k.0, k.1 + 1),
                (k.0 + 1, k.1),
                (k.0 + 1, k.1 - 1),
                (k.0, k.1 - 1),
                (k.0 - 1, k.1),
                (k.0 - 1, k.1 + 1),
            ];
            neighbours.iter().for_each(|&neighbour| {
                new_tiles.entry(neighbour).or_insert(0);
            });
        });
        tiles = new_tiles.clone();

        tiles.iter().for_each(|(&k, v)| {
            let neighbours = [
                (k.0, k.1 + 1),
                (k.0 + 1, k.1),
                (k.0 + 1, k.1 - 1),
                (k.0, k.1 - 1),
                (k.0 - 1, k.1),
                (k.0 - 1, k.1 + 1),
            ];

            let adjacent_black = neighbours
                .iter()
                .filter(|&k| {
                    if let Some(neighbour) = tiles.get(k) {
                        neighbour % 2 != 0
                    } else {
                        false
                    }
                })
                .count();

            if v % 2 != 0 {
                if adjacent_black == 0 || adjacent_black > 2 {
                    let entry = new_tiles.entry(k).or_insert(0);
                    *entry = 0;
                }
            } else if adjacent_black == 2 {
                let entry = new_tiles.entry(k).or_insert(0);
                *entry = 1;
            }
        });

        tiles = new_tiles;
    }

    tiles.values().filter(|&v| v % 2 != 0).count()
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let tiles = parse_input(&input);

    assert_eq!(part_1(&tiles), 277);
    assert_eq!(part_2(tiles), 3_531);
}
