use std::fs;
use std::mem;

enum Direction {
    North,
    South,
    East,
    West,
}

struct Ship {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::East,
        }
    }

    fn move_north(&mut self, value: i32) {
        self.y -= value;
    }

    fn move_south(&mut self, value: i32) {
        self.y += value;
    }

    fn move_east(&mut self, value: i32) {
        self.x += value;
    }

    fn move_west(&mut self, value: i32) {
        self.x -= value;
    }

    fn turn_left(&mut self, value: i32) {
        let rotations = value / 90;
        (0..rotations).for_each(|_| match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::South => self.direction = Direction::East,
            Direction::East => self.direction = Direction::North,
            Direction::West => self.direction = Direction::South,
        });
    }

    fn turn_right(&mut self, value: i32) {
        let rotations = value / 90;
        (0..rotations).for_each(|_| match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::South => self.direction = Direction::West,
            Direction::East => self.direction = Direction::South,
            Direction::West => self.direction = Direction::North,
        });
    }

    fn move_forward(&mut self, value: i32) {
        match self.direction {
            Direction::North => self.y -= value,
            Direction::South => self.y += value,
            Direction::East => self.x += value,
            Direction::West => self.x -= value,
        }
    }
}

struct ShipAndWaypoint {
    ship_x: i32,
    ship_y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl ShipAndWaypoint {
    fn new() -> Self {
        Self {
            ship_x: 0,
            ship_y: 0,
            waypoint_x: 10,
            waypoint_y: 1,
        }
    }

    fn move_waypoint_north(&mut self, value: i32) {
        self.waypoint_y += value;
    }

    fn move_waypoint_south(&mut self, value: i32) {
        self.waypoint_y -= value;
    }

    fn move_waypoint_east(&mut self, value: i32) {
        self.waypoint_x += value;
    }

    fn move_waypoint_west(&mut self, value: i32) {
        self.waypoint_x -= value;
    }

    fn turn_waypoint_left(&mut self, value: i32) {
        let rotations = value / 90;
        (0..rotations).for_each(|_| {
            mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
            self.waypoint_x *= -1;
        });
    }

    fn turn_waypoint_right(&mut self, value: i32) {
        let rotations = value / 90;
        (0..rotations).for_each(|_| {
            mem::swap(&mut self.waypoint_x, &mut self.waypoint_y);
            self.waypoint_y *= -1;
        });
    }

    fn move_towards_waypoint(&mut self, value: i32) {
        self.ship_x += value * self.waypoint_x;
        self.ship_y += value * self.waypoint_y;
    }
}

fn part_1(instructions: &str) -> i32 {
    let mut ship = Ship::new();

    for instruction in instructions.lines() {
        let (action, value) = instruction.split_at(1);
        let value = value.parse::<i32>().unwrap();
        match action {
            "N" => ship.move_north(value),
            "S" => ship.move_south(value),
            "E" => ship.move_east(value),
            "W" => ship.move_west(value),
            "L" => ship.turn_left(value),
            "R" => ship.turn_right(value),
            "F" => ship.move_forward(value),
            _ => panic!("unknown action"),
        }
    }

    ship.x + ship.y
}

fn part_2(instructions: &str) -> i32 {
    let mut ship_waypoint = ShipAndWaypoint::new();

    for instruction in instructions.lines() {
        let (action, value) = instruction.split_at(1);
        let value = value.parse::<i32>().unwrap();

        match action {
            "N" => ship_waypoint.move_waypoint_north(value),
            "S" => ship_waypoint.move_waypoint_south(value),
            "E" => ship_waypoint.move_waypoint_east(value),
            "W" => ship_waypoint.move_waypoint_west(value),
            "L" => ship_waypoint.turn_waypoint_left(value),
            "R" => ship_waypoint.turn_waypoint_right(value),
            "F" => ship_waypoint.move_towards_waypoint(value),
            _ => panic!("unknown action"),
        }
    }

    ship_waypoint.ship_x + ship_waypoint.ship_y
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    assert_eq!(part_1(&content), 2_270);
    assert_eq!(part_2(&content), 138_669);
}
