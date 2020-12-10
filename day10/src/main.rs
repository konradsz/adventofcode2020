use std::fs;

fn count(current_index: usize, ratings: &[usize], used: &[bool]) -> usize {
    let mut total = 0;
    for step in &[1, 2, 3] {
        let next_index = current_index + step;
        if next_index > (ratings.len() - 1)
            || used[next_index]
            || (ratings[current_index] > ratings[next_index]
                && ratings[current_index] - ratings[next_index] > 3)
            || (ratings[current_index] < ratings[next_index]
                && ratings[next_index] - ratings[current_index] > 3)
        {
            continue;
        }

        if next_index == (ratings.len() - 1) {
            total += 1;
        } else {
            let mut new_used = used.to_owned();
            new_used[next_index] = true;

            total += count(next_index, ratings, &new_used);
        }
    }

    total
}

fn part_1(ratings: &[usize]) -> usize {
    let r = ratings.windows(2).fold((0, 0), |acc, w| {
        if w[1] - w[0] == 1 {
            (acc.0 + 1, acc.1)
        } else {
            (acc.0, acc.1 + 1)
        }
    });

    r.0 * r.1
}

fn part_2(ratings: &[usize]) -> usize {
    let mut anchored = vec![false; ratings.len()];
    anchored[0] = true;
    ratings
        .iter()
        .enumerate()
        .zip(ratings.iter().enumerate().skip(1))
        .for_each(|((i1, v1), (i2, v2))| {
            if v2 - v1 == 3 {
                anchored[i1] = true;
                anchored[i2] = true;
            }
        });

    let mut ranges = vec![];
    let mut index = 0;
    while index < ratings.len() {
        if !anchored[index] {
            let mut end_index = index;
            while !anchored[end_index] {
                end_index += 1;
            }

            ranges.push((index - 1, end_index + 1));
            index = end_index;
        } else {
            index += 1;
        }
    }

    ranges
        .iter()
        .map(|r| {
            let mut used = vec![false; r.1 - r.0];
            used[0] = true;
            count(0, &ratings[r.0..r.1], &used)
        })
        .product()
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let mut ratings = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    ratings.push(0);
    ratings.sort_unstable();
    ratings.push(ratings.last().unwrap() + 3);

    assert_eq!(part_1(&ratings), 1_984);
    assert_eq!(part_2(&ratings), 3_543_369_523_456);
}
