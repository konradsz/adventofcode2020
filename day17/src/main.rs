use std::collections::HashMap;
use std::fs;

const CYCLES: usize = 6;

#[derive(PartialEq, Copy, Clone)]
enum State {
    Active,
    Inactive,
}

struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn extend(&mut self) {
        self.min -= 1;
        self.max += 1;
    }
}

fn part_1(mut grid: HashMap<(i32, i32, i32), State>) -> usize {
    let mut x_range = Range { min: 0, max: 8 };
    let mut y_range = Range { min: 0, max: 8 };
    let mut z_range = Range { min: 0, max: 0 };

    for _ in 0..CYCLES {
        x_range.extend();
        y_range.extend();
        z_range.extend();

        for x in x_range.min..x_range.max {
            for y in y_range.min..y_range.max {
                for z in z_range.min..=z_range.max {
                    grid.entry((x, y, z)).or_insert(State::Inactive);
                }
            }
        }

        let mut next_grid = grid.clone();

        for (coords, state) in grid.iter() {
            let mut active_neighbours = 0;
            for &n_x in [-1, 0, 1].iter() {
                for &n_y in [-1, 0, 1].iter() {
                    for &n_z in [-1, 0, 1].iter() {
                        if n_x == 0 && n_y == 0 && n_z == 0 {
                            continue;
                        }

                        if let Some(neighbour) =
                            grid.get(&(coords.0 + n_x, coords.1 + n_y, coords.2 + n_z))
                        {
                            if neighbour == &State::Active {
                                active_neighbours += 1;
                            }
                        }
                    }
                }
            }

            if state == &State::Active && active_neighbours != 2 && active_neighbours != 3 {
                let s = next_grid.get_mut(coords).unwrap();
                *s = State::Inactive;
            } else if state == &State::Inactive && active_neighbours == 3 {
                let s = next_grid.get_mut(coords).unwrap();
                *s = State::Active;
            }
        }

        grid = next_grid;
    }

    grid.values()
        .filter(|&state| state == &State::Active)
        .count()
}

fn part_2(mut grid: HashMap<(i32, i32, i32, i32), State>) -> usize {
    let mut x_range = Range { min: 0, max: 8 };
    let mut y_range = Range { min: 0, max: 8 };
    let mut z_range = Range { min: 0, max: 0 };
    let mut w_range = Range { min: 0, max: 0 };

    for _ in 0..CYCLES {
        x_range.extend();
        y_range.extend();
        z_range.extend();
        w_range.extend();

        for x in x_range.min..x_range.max {
            for y in y_range.min..y_range.max {
                for z in z_range.min..=z_range.max {
                    for w in w_range.min..=w_range.max {
                        grid.entry((x, y, z, w)).or_insert(State::Inactive);
                    }
                }
            }
        }

        let mut next_grid = grid.clone();

        for (coords, state) in grid.iter() {
            let mut active_neighbours = 0;
            for &n_x in [-1, 0, 1].iter() {
                for &n_y in [-1, 0, 1].iter() {
                    for &n_z in [-1, 0, 1].iter() {
                        for &n_w in [-1, 0, 1].iter() {
                            if n_x == 0 && n_y == 0 && n_z == 0 && n_w == 0 {
                                continue;
                            }

                            if let Some(neighbour) = grid.get(&(
                                coords.0 + n_x,
                                coords.1 + n_y,
                                coords.2 + n_z,
                                coords.3 + n_w,
                            )) {
                                if neighbour == &State::Active {
                                    active_neighbours += 1;
                                }
                            }
                        }
                    }
                }
            }

            if state == &State::Active && active_neighbours != 2 && active_neighbours != 3 {
                let s = next_grid.get_mut(coords).unwrap();
                *s = State::Inactive;
            } else if state == &State::Inactive && active_neighbours == 3 {
                let s = next_grid.get_mut(coords).unwrap();
                *s = State::Active;
            }
        }

        grid = next_grid;
    }

    grid.values()
        .filter(|&state| state == &State::Active)
        .count()
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let mut grid_3d = HashMap::new();
    let mut grid_4d = HashMap::new();
    content.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let state = match c {
                '.' => State::Inactive,
                '#' => State::Active,
                _ => panic!("unrecognized character"),
            };
            grid_3d.insert((x as i32, y as i32, 0), state);
            grid_4d.insert((x as i32, y as i32, 0, 0), state);
        });
    });

    assert_eq!(part_1(grid_3d), 324);
    assert_eq!(part_2(grid_4d), 1_836);
}
