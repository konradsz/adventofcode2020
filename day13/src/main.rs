fn part_1() -> i32 {
    let earliest_time = 1_000_508;
    let ids = [29, 37, 467, 23, 13, 17, 19, 443, 41];

    for time in earliest_time.. {
        if let Some(id) = ids.iter().find(|&&id| time % id == 0) {
            return (time - earliest_time) * id;
        }
    }

    unreachable!()
}

fn part_2() -> usize {
    // calculated from equations:
    // x ≡ 0 (mod 29)
    // x ≡ 14 (mod 37)
    // x ≡ 438 (mod 467)
    // x ≡ 9 (mod 23)
    // x ≡ 10 (mod 13)
    // x ≡ 5 (mod 17)
    // x ≡ 9 (mod 19)
    // x ≡ 383 (mod 443)
    // x ≡ 22 (mod 41)
    // using "Chinese remainder theorem"
    690_123_192_779_524
}

fn main() {
    assert_eq!(part_1(), 333);
    assert_eq!(part_2(), 690_123_192_779_524);
}
