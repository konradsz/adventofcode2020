use std::collections::VecDeque;
use std::fs;

fn calculate_score(cards: &VecDeque<usize>) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + (i + 1) * x)
}

fn main() {
    let player_1_cards = fs::read_to_string("input_player_1").unwrap();
    let player_2_cards = fs::read_to_string("input_player_2").unwrap();

    let mut player_1_cards = player_1_cards
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();
    let mut player_2_cards = player_2_cards
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();

    while !player_1_cards.is_empty() && !player_2_cards.is_empty() {
        let player_1_card = player_1_cards.pop_front().unwrap();
        let player_2_card = player_2_cards.pop_front().unwrap();

        if player_1_card > player_2_card {
            player_1_cards.push_back(player_1_card);
            player_1_cards.push_back(player_2_card);
        } else {
            player_2_cards.push_back(player_2_card);
            player_2_cards.push_back(player_1_card);
        }
    }

    let score = if player_1_cards.is_empty() {
        calculate_score(&player_2_cards)
    } else {
        calculate_score(&player_1_cards)
    };

    println!("{}", score);
}
