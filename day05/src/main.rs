use std::fs;

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn get_lower_half(&mut self) {
        self.max = (self.min + self.max) / 2;
    }

    fn get_upper_half(&mut self) {
        self.min = (self.min + self.max + 1) / 2;
    }
}

fn get_row(boarding_pass: &str) -> u32 {
    boarding_pass
        .chars()
        .take(7)
        .fold(&mut Range { min: 0, max: 127 }, |r, c| {
            match c {
                'F' => r.get_lower_half(),
                'B' => r.get_upper_half(),
                _ => panic!(),
            };
            r
        })
        .min
}

fn get_column(boarding_pass: &str) -> u32 {
    boarding_pass
        .chars()
        .skip(7)
        .take(3)
        .fold(&mut Range { min: 0, max: 7 }, |r, c| {
            match c {
                'L' => r.get_lower_half(),
                'R' => r.get_upper_half(),
                _ => panic!(),
            };
            r
        })
        .min
}

fn part_1(seat_ids: &[u32]) -> u32 {
    *seat_ids.last().unwrap()
}

fn part_2(seat_ids: &[u32]) -> u32 {
    seat_ids
        .iter()
        .zip(seat_ids.iter().skip(1))
        .find(|pair| (pair.1 - pair.0) > 1)
        .unwrap()
        .0
        + 1
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    let mut seat_ids: Vec<u32> = content
        .lines()
        .map(|boarding_pass| get_row(boarding_pass) * 8 + get_column(boarding_pass))
        .collect();
    seat_ids.sort_unstable();

    assert_eq!(part_1(&seat_ids), 906);
    assert_eq!(part_2(&seat_ids), 519);
}
