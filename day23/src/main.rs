fn play_game(cups: &mut Vec<usize>, mut current_cup: usize, rounds: usize) {
    let last_index = cups.len() - 1;
    for _ in 0..rounds {
        let cups_taken = [
            cups[current_cup],
            cups[cups[current_cup]],
            cups[cups[cups[current_cup]]],
        ];

        let mut destination = current_cup;

        loop {
            destination -= 1;
            if destination == 0 {
                destination = last_index;
            }

            if !cups_taken.contains(&&destination) {
                break;
            }
        }
        cups[current_cup] = cups[cups_taken[2]];

        cups[cups_taken[2]] = cups[destination];
        cups[destination] = cups_taken[0];

        current_cup = cups[current_cup];
    }
}

fn part_1() -> usize {
    let mut cups = vec![0, 4, 5, 8, 7, 3, 2, 6, 9, 1];
    let current_cup = 5;

    play_game(&mut cups, current_cup, 100);

    let mut result = String::with_capacity(8);
    let mut current_cup = cups[1];
    for _ in 0..8 {
        result.push_str(&current_cup.to_string());
        current_cup = cups[current_cup];
    }
    result.parse().unwrap()
}

fn part_2() -> usize {
    let mut cups = vec![0, 4, 10, 8, 7, 3, 2, 6, 9, 1];

    for i in 11..1_000_001 {
        cups.push(i);
    }
    cups.push(5);
    let current_cup = 5;

    play_game(&mut cups, current_cup, 10_000_000);

    cups[1] * cups[cups[1]]
}

fn main() {
    assert_eq!(part_1(), 54327968);
    assert_eq!(part_2(), 157_410_423_276);
}
