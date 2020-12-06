use std::fs;
struct Range(u32, u32);

fn get_seat<I: Iterator<Item = char>>(vals: I, max: u32) -> u32 {
    vals.fold(&mut Range(0, max), |range, c| {
        match c {
            'F' | 'L' => range.1 = (range.0 + range.1) / 2,
            'B' | 'R' => range.0 = (range.0 + range.1 + 1) / 2,
            _ => panic!(),
        };
        range
    })
    .0
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
        .map(|boarding_pass| {
            get_seat(boarding_pass.chars().take(7), 127) * 8
                + get_seat(boarding_pass.chars().skip(7).take(3), 7)
        })
        .collect();
    seat_ids.sort_unstable();

    assert_eq!(part_1(&seat_ids), 906);
    assert_eq!(part_2(&seat_ids), 519);
}
