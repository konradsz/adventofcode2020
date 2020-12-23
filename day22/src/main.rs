use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

enum Winner {
    Player1,
    Player2,
}

fn calculate_score(cards: &VecDeque<usize>) -> usize {
    cards
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc + (i + 1) * x)
}

fn play_game(
    mut player_1_cards: VecDeque<usize>,
    mut player_2_cards: VecDeque<usize>,
) -> (Winner, usize) {
    let mut current_game_history = HashSet::new();
    while !player_1_cards.is_empty() && !player_2_cards.is_empty() {
        if current_game_history
            .get(&(player_1_cards.clone(), player_2_cards.clone()))
            .is_some()
        {
            return (Winner::Player1, calculate_score(&player_1_cards));
        } else {
            current_game_history.insert((player_1_cards.clone(), player_2_cards.clone()));
        }

        let player_1_card = player_1_cards.pop_front().unwrap();
        let player_2_card = player_2_cards.pop_front().unwrap();

        if player_1_card > player_1_cards.len() || player_2_card > player_2_cards.len() {
            if player_1_card > player_2_card {
                player_1_cards.push_back(player_1_card);
                player_1_cards.push_back(player_2_card);
            } else {
                player_2_cards.push_back(player_2_card);
                player_2_cards.push_back(player_1_card);
            }
        } else {
            let new_player_1_cards = player_1_cards
                .iter()
                .take(player_1_card)
                .cloned()
                .collect::<VecDeque<usize>>();

            let new_player_2_cards = player_2_cards
                .iter()
                .take(player_2_card)
                .cloned()
                .collect::<VecDeque<usize>>();

            match play_game(new_player_1_cards, new_player_2_cards).0 {
                Winner::Player1 => {
                    player_1_cards.push_back(player_1_card);
                    player_1_cards.push_back(player_2_card);
                }
                Winner::Player2 => {
                    player_2_cards.push_back(player_2_card);
                    player_2_cards.push_back(player_1_card);
                }
            }
        }
    }

    if player_1_cards.is_empty() {
        (Winner::Player2, calculate_score(&player_2_cards))
    } else {
        (Winner::Player1, calculate_score(&player_1_cards))
    }
}

fn part_1(mut player_1_cards: VecDeque<usize>, mut player_2_cards: VecDeque<usize>) -> usize {
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

    if player_1_cards.is_empty() {
        calculate_score(&player_2_cards)
    } else {
        calculate_score(&player_1_cards)
    }
}

fn part_2(player_1_cards: VecDeque<usize>, player_2_cards: VecDeque<usize>) -> usize {
    play_game(player_1_cards, player_2_cards).1
}

fn main() {
    let player_1_cards = fs::read_to_string("input_player_1").unwrap();
    let player_2_cards = fs::read_to_string("input_player_2").unwrap();

    let player_1_cards = player_1_cards
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();
    let player_2_cards = player_2_cards
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();

    assert_eq!(
        part_1(player_1_cards.clone(), player_2_cards.clone()),
        32_598
    );
    assert_eq!(part_2(player_1_cards, player_2_cards), 35_836);
}
