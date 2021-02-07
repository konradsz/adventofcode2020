use std::collections::{HashMap, HashSet};
use std::fs;

type Orientation = (bool, u8);
const ORIENTATIONS: [Orientation; 8] = [
    (false, 0),
    (false, 1),
    (false, 2),
    (false, 3),
    (true, 0),
    (true, 1),
    (true, 2),
    (true, 3),
];

#[derive(Clone)]
struct Tile {
    id: u32,
    data: Vec<String>,
}

impl Tile {
    fn new(id: u32, data: Vec<String>) -> Self {
        Self { id, data }
    }

    fn left(&self) -> String {
        self.data
            .iter()
            .map(|line| line.chars().next().unwrap())
            .collect::<String>()
    }

    fn top(&self) -> String {
        self.data.first().unwrap().clone()
    }

    fn right(&self) -> String {
        self.data
            .iter()
            .map(|line| line.chars().last().unwrap())
            .collect::<String>()
    }

    fn bottom(&self) -> String {
        self.data.last().unwrap().clone()
    }

    fn rotate(&mut self) {
        let mut new_data = Vec::with_capacity(self.data.len());
        let length = self.data.first().unwrap().len();
        (0..length).for_each(|i| {
            new_data.push(
                self.data
                    .iter()
                    .rev()
                    .map(|line| line.chars().nth(i).unwrap())
                    .collect::<String>(),
            );
        });
        self.data = new_data;
    }

    fn flip(&mut self) {
        self.data
            .iter_mut()
            .for_each(|line| *line = line.chars().rev().collect::<String>());
    }

    fn transform(&self, orientation: &Orientation) -> Self {
        let mut new_tile = self.clone();
        if orientation.0 {
            new_tile.flip();
        }
        (0..orientation.1).for_each(|_| new_tile.rotate());

        new_tile
    }
}

type Picture = HashMap<(i32, i32), Tile>;

fn reassemble_picture(tiles: &[Tile]) -> Picture {
    let mut possible_neighbours = HashSet::new();
    possible_neighbours.insert((0, 1));
    possible_neighbours.insert((0, -1));
    possible_neighbours.insert((1, 0));
    possible_neighbours.insert((-1, 0));

    let mut picture = HashMap::new();
    picture.insert((0, 0), tiles[0].clone());

    if !find_neighbours(tiles, &mut picture, possible_neighbours) {
        panic!("Cannot assemble picture!");
    }

    picture
}

fn find_neighbours(
    tiles: &[Tile],
    picture: &mut HashMap<(i32, i32), Tile>,
    mut possible_neighbours: HashSet<(i32, i32)>,
) -> bool {
    while !possible_neighbours.is_empty() {
        let coordinates = *possible_neighbours.iter().next().unwrap();
        possible_neighbours.remove(&coordinates);

        if picture.get(&coordinates).is_some() {
            continue;
        }

        for tile in tiles {
            if picture
                .values()
                .any(|assembled_tile| assembled_tile.id == tile.id)
            {
                continue;
            }

            for orientation in &ORIENTATIONS {
                let tile_candidate = tile.transform(orientation);

                if let Some(left_neighbour) = picture.get(&(coordinates.0 - 1, coordinates.1)) {
                    if left_neighbour.right() != tile_candidate.left() {
                        continue;
                    }
                }
                if let Some(top_neighbour) = picture.get(&(coordinates.0, coordinates.1 - 1)) {
                    if top_neighbour.bottom() != tile_candidate.top() {
                        continue;
                    }
                }
                if let Some(right_neighbour) = picture.get(&(coordinates.0 + 1, coordinates.1)) {
                    if right_neighbour.left() != tile_candidate.right() {
                        continue;
                    }
                }
                if let Some(bottom_neighbour) = picture.get(&(coordinates.0, coordinates.1 + 1)) {
                    if bottom_neighbour.top() != tile_candidate.bottom() {
                        continue;
                    }
                }

                picture.insert(coordinates, tile_candidate);

                let mut new_possible_neighbours = possible_neighbours.clone();
                new_possible_neighbours.insert((coordinates.0 + 1, coordinates.1));
                new_possible_neighbours.insert((coordinates.0 - 1, coordinates.1));
                new_possible_neighbours.insert((coordinates.0, coordinates.1 + 1));
                new_possible_neighbours.insert((coordinates.0, coordinates.1 - 1));

                if find_neighbours(tiles, picture, new_possible_neighbours) {
                    return true;
                }

                picture.remove(&coordinates);
            }
        }
    }

    picture.len() == tiles.len()
}

fn part_1(picture: &Picture) -> usize {
    let min_y = picture.keys().map(|k| k.0).min().unwrap();
    let max_y = picture.keys().map(|k| k.0).max().unwrap();
    let min_x = picture.keys().map(|k| k.1).min().unwrap();
    let max_x = picture.keys().map(|k| k.1).max().unwrap();

    let top_left_id = picture.get(&(min_y, min_x)).unwrap().id;
    let top_right_id = picture.get(&(min_y, max_x)).unwrap().id;
    let bottom_left_id = picture.get(&(max_y, min_x)).unwrap().id;
    let bottom_right_id = picture.get(&(max_y, max_x)).unwrap().id;

    top_left_id as usize
        * top_right_id as usize
        * bottom_left_id as usize
        * bottom_right_id as usize
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let mut tiles = Vec::new();
    let mut tile_data = Vec::new();
    let mut tile_id = 0;
    for line in content.lines() {
        if line.contains("Tile") {
            tile_id = line
                .trim_start_matches("Tile ")
                .trim_end_matches(':')
                .parse::<u32>()
                .unwrap();
        } else if line.is_empty() {
            tile_data.clear();
        } else {
            tile_data.push(line.to_owned());
        }

        if tile_data.len() == 10 {
            tiles.push(Tile::new(tile_id, tile_data));
            tile_data = Vec::new();
        }
    }

    let picture = reassemble_picture(&tiles);
    assert_eq!(108_603_771_107_737, part_1(&picture));
}
