use std::fs;

#[derive(PartialEq)]
enum Tile {
    Open,
    Tree,
}

fn part_1(map: &[Vec<Tile>]) -> usize {
    let width = map.first().unwrap().len();

    map.iter()
        .step_by(1)
        .enumerate()
        .filter(|(h, row)| row[h * 3 % width] == Tile::Tree)
        .count()
}

fn part_2(map: &[Vec<Tile>]) -> usize {
    let width = map.first().unwrap().len();
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes
        .iter()
        .map(|slope| {
            map.iter()
                .step_by(slope.1)
                .enumerate()
                .filter(|(h, row)| row[h * slope.0 % width] == Tile::Tree)
                .count()
        })
        .product()
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let map: Vec<Vec<Tile>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Open,
                    '#' => Tile::Tree,
                    _ => panic!("unknown tile type"),
                })
                .collect::<Vec<Tile>>()
        })
        .collect();

    assert_eq!(part_1(&map), 237);
    assert_eq!(part_2(&map), 2_106_818_610);
}
