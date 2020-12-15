use std::collections::HashMap;

fn get_nth_spoken_number(mut number_history: HashMap<usize, usize>, n: usize) -> usize {
    let mut current_number = 0;

    for current_turn in number_history.len() + 1..n {
        if let Some(last_turn) = number_history.get_mut(&current_number) {
            current_number = current_turn - *last_turn;
            *last_turn = current_turn;
        } else {
            number_history.insert(current_number, current_turn);
            current_number = 0;
        }
    }

    current_number
}

fn main() {
    let mut number_history = HashMap::new();
    number_history.insert(14, 1);
    number_history.insert(1, 2);
    number_history.insert(17, 3);
    number_history.insert(0, 4);
    number_history.insert(3, 5);
    number_history.insert(20, 6);

    assert_eq!(get_nth_spoken_number(number_history.clone(), 2020), 387); // part_1
    assert_eq!(get_nth_spoken_number(number_history, 30_000_000), 6428); // part_2
}
