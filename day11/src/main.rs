use std::fs;

const GRID_WIDTH: usize = 98;
const GRID_HEIGHT: usize = 97;

#[derive(PartialEq)]
enum Field {
    Floor,
    Empty,
    Occupied,
}

type Layout = Vec<Vec<Field>>;

fn print(layout: &Layout) {
    for row in layout {
        for field in row {
            match field {
                Field::Floor => print!("."),
                Field::Empty => print!("L"),
                Field::Occupied => print!("#"),
            }
        }
        println!();
    }
}

fn has_neighbour(x: usize, y: usize, offset: (i32, i32)) -> bool {
    (offset.0 >= 0 || x != 0)
        && (offset.0 <= 0 || x != GRID_WIDTH - 1)
        && (offset.1 >= 0 || y != 0)
        && (offset.1 <= 0 || y != GRID_HEIGHT - 1)
}

fn count_adjacent(x: usize, y: usize, layout: &Layout, status: Field) -> usize {
    // count_occupied_around
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
        layout[(y as i32 + offset.1) as usize][(x as i32 + offset.0) as usize] == status
    })
    .count()
}

fn process(layout: &Layout) -> Layout {
    let mut new_layout = vec![];

    for (y, row) in layout.iter().enumerate() {
        let mut new_row = vec![];
        for (x, field) in row.iter().enumerate() {
            match field {
                Field::Floor => new_row.push(Field::Floor),
                Field::Empty => {
                    if count_adjacent(x, y, layout, Field::Occupied) == 0 {
                        new_row.push(Field::Occupied)
                    } else {
                        new_row.push(Field::Empty)
                    }
                }
                Field::Occupied => {
                    if count_adjacent(x, y, layout, Field::Occupied) >= 4 {
                        new_row.push(Field::Empty)
                    } else {
                        new_row.push(Field::Occupied)
                    }
                }
            }
        }
        new_layout.push(new_row);
    }

    new_layout
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");

    let mut layout = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Field::Floor,
                    'L' => Field::Empty,
                    '#' => Field::Occupied,
                    _ => panic!("incorrect field"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    loop {
        let new = process(&layout);
        if new == layout {
            let occupied: usize = new
                .iter()
                .map(|row| row.iter().filter(|&f| f == &Field::Occupied).count())
                .sum();
            println!("{}", occupied);
            break;
        } else {
            layout = new;
        }
    }
}
