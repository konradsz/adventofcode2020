use itertools::Itertools;
use std::fs;

fn part_1(numbers: &[usize]) -> usize {
    const PREAMBLE_SIZE: usize = 25;
    for i in PREAMBLE_SIZE..numbers.len() {
        let preamble = &numbers[i - PREAMBLE_SIZE..i];
        let mut perms = preamble.iter().permutations(2);
        if perms
            .find(|p| p.iter().cloned().sum::<usize>() == numbers[i])
            .is_none()
        {
            return numbers[i];
        }
    }
    unreachable!()
}

fn part_2(numbers: &[usize]) -> usize {
    const SUM: usize = 133_015_568;
    for starting_index in 0..numbers.len() {
        let mut current_sum = 0;
        for current_index in starting_index..numbers.len() {
            current_sum += numbers[current_index];
            if current_sum == SUM {
                let slice = &numbers[starting_index..current_index];
                let min = slice.iter().min().unwrap();
                let max = slice.iter().max().unwrap();
                return min + max;
            } else if current_sum > SUM {
                break;
            }
        }
    }
    unreachable!()
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let numbers: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    assert_eq!(part_1(&numbers), 133_015_568);
    assert_eq!(part_2(&numbers), 16_107_959);
}
