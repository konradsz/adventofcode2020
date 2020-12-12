use std::fs;

const GRID_WIDTH: usize = 98;
const GRID_HEIGHT: usize = 97;

#[derive(Copy, Clone, PartialEq)]
enum Field {
    Floor,
    Empty,
    Occupied,
}

type Layout = [Field];

fn has_neighbour(x: usize, y: usize, offset: (i32, i32)) -> bool {
    (offset.0 >= 0 || x != 0)
        && (offset.0 <= 0 || x != GRID_WIDTH - 1)
        && (offset.1 >= 0 || y != 0)
        && (offset.1 <= 0 || y != GRID_HEIGHT - 1)
}

fn count_occupied_adjacent(x: usize, y: usize, layout: &Layout) -> usize {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    .filter(|offset| has_neighbour(x, y, **offset))
    .filter(|offset| {
        layout[(y as i32 + offset.1) as usize * GRID_WIDTH + (x as i32 + offset.0) as usize]
            == Field::Occupied
    })
    .count()
}

fn count_occupied_visible(x: usize, y: usize, layout: &Layout) -> usize {
    [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ]
    .iter()
    .filter(|dir| {
        let mut next = (x as i32, y as i32);
        loop {
            next.0 += dir.0;
            next.1 += dir.1;
            if next.0 < 0
                || next.0 as usize > GRID_WIDTH - 1
                || next.1 < 0
                || next.1 as usize > GRID_HEIGHT - 1
            {
                break;
            }

            if layout[next.1 as usize * GRID_WIDTH + next.0 as usize] == Field::Occupied {
                return true;
            } else if layout[next.1 as usize * GRID_WIDTH + next.0 as usize] == Field::Empty {
                break;
            }
        }

        false
    })
    .count()
}

fn shift_part_1(layout: &Layout) -> Vec<Field> {
    let mut new_layout = Vec::with_capacity(GRID_HEIGHT * GRID_WIDTH);

    for (index, field) in layout.iter().enumerate() {
        let x = index % GRID_WIDTH;
        let y = index / GRID_WIDTH;

        match field {
            Field::Floor => new_layout.push(Field::Floor),
            Field::Empty => {
                if count_occupied_adjacent(x, y, layout) == 0 {
                    new_layout.push(Field::Occupied)
                } else {
                    new_layout.push(Field::Empty)
                }
            }
            Field::Occupied => {
                if count_occupied_adjacent(x, y, layout) >= 4 {
                    new_layout.push(Field::Empty)
                } else {
                    new_layout.push(Field::Occupied)
                }
            }
        }
    }

    new_layout
}

fn shift_part_2(layout: &Layout) -> Vec<Field> {
    let mut new_layout = Vec::with_capacity(GRID_HEIGHT * GRID_WIDTH);

    for (index, field) in layout.iter().enumerate() {
        let x = index % GRID_WIDTH;
        let y = index / GRID_WIDTH;

        match field {
            Field::Floor => new_layout.push(Field::Floor),
            Field::Empty => {
                if count_occupied_visible(x, y, layout) == 0 {
                    new_layout.push(Field::Occupied)
                } else {
                    new_layout.push(Field::Empty)
                }
            }
            Field::Occupied => {
                if count_occupied_visible(x, y, layout) >= 5 {
                    new_layout.push(Field::Empty)
                } else {
                    new_layout.push(Field::Occupied)
                }
            }
        }
    }

    new_layout
}

fn run_until_stabilizes_and_count_occupied(
    layout: &Layout,
    shift: fn(&Layout) -> Vec<Field>,
) -> usize {
    let mut previous = layout.to_owned();
    loop {
        let new = shift(&previous);
        if new == previous {
            return new.iter().filter(|&&f| f == Field::Occupied).count();
        }
        previous = new;
    }
}

fn part_1(layout: &Layout) -> usize {
    run_until_stabilizes_and_count_occupied(layout, shift_part_1)
}

fn part_2(layout: &Layout) -> usize {
    run_until_stabilizes_and_count_occupied(layout, shift_part_2)
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");

    let layout = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '.' => Field::Floor,
            'L' => Field::Empty,
            '#' => Field::Occupied,
            _ => panic!("incorrect field"),
        })
        .collect::<Vec<_>>();

    assert_eq!(part_1(&layout), 2_481);
    assert_eq!(part_2(&layout), 2_227);
}
