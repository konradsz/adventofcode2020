use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn part_1(program: &str) -> usize {
    let mut memory = HashMap::new();

    let mut and_mask = 0;
    let mut or_mask = 0;
    for instruction in program.lines() {
        if instruction.contains("mask") {
            let m = instruction.split_at(7).1;
            let and = m.replace("X", "1");
            and_mask = usize::from_str_radix(&and, 2).unwrap();

            let or = m.replace("X", "0");
            or_mask = usize::from_str_radix(&or, 2).unwrap();
        } else if instruction.contains("mem") {
            let parts = instruction.split_terminator('=').collect::<Vec<_>>();
            let address = parts[0]
                .trim_matches(|c: char| !c.is_digit(10))
                .parse::<usize>()
                .unwrap();
            let value = parts[1]
                .trim_matches(|c: char| !c.is_digit(10))
                .parse::<usize>()
                .unwrap();

            let e = memory.entry(address).or_default();
            *e = (value & and_mask) | or_mask;
        }
    }

    memory.values().sum::<usize>()
}

#[derive(Copy, Clone, PartialEq)]
enum Bit {
    Zero,
    One,
    Floating,
}

fn parse_mask(mask: &str) -> Vec<Bit> {
    mask.chars()
        .map(|c| match c {
            '0' => Bit::Zero,
            '1' => Bit::One,
            'X' => Bit::Floating,
            _ => panic!("unrecognized character in mask"),
        })
        .collect::<Vec<_>>()
}

fn apply_mask(address: usize, mask: &[Bit]) -> Vec<Bit> {
    let mut address_as_bits = VecDeque::new();

    let mut rest = address;

    let mut offset = 0;
    while rest > 0 {
        let bit = (address & (1 << offset)) >> offset;
        match bit {
            0 => address_as_bits.push_front(Bit::Zero),
            1 => address_as_bits.push_front(Bit::One),
            _ => unreachable!(),
        };

        offset += 1;

        rest = address >> offset;
    }

    let mut applied = mask
        .iter()
        .rev()
        .zip(address_as_bits.iter().rev())
        .map(|(mask_bit, address_bit)| match mask_bit {
            Bit::Zero => *address_bit,
            Bit::One => Bit::One,
            Bit::Floating => Bit::Floating,
        })
        .collect::<Vec<_>>();

    if address_as_bits.len() > mask.len() {
        address_as_bits
            .iter()
            .rev()
            .skip(mask.len())
            .for_each(|b| applied.push(*b));
    } else if address_as_bits.len() < mask.len() {
        mask.iter()
            .rev()
            .skip(address_as_bits.len())
            .for_each(|b| applied.push(*b));
    }

    applied.reverse();
    applied
}

fn generate_possibilities(address: Vec<Bit>) -> Vec<usize> {
    let floatings_bits = address.iter().filter(|&b| b == &Bit::Floating).count();
    let mut result = vec![address];

    for _ in 0..floatings_bits {
        let mut next_results = vec![];
        let current_index = result[0].iter().position(|&b| b == Bit::Floating).unwrap();
        for r in result.iter() {
            let mut replaced = r.clone();
            replaced[current_index] = Bit::One;
            next_results.push(replaced.clone());
            replaced[current_index] = Bit::Zero;
            next_results.push(replaced);
        }

        result = next_results;
    }

    result
        .iter()
        .map(|address| {
            let mut a = 0;
            for bit in address {
                if matches!(bit, Bit::One) {
                    a |= 1;
                }
                a <<= 1;
            }
            a >>= 1;
            a
        })
        .collect()
}

fn part_2(program: &str) -> usize {
    let mut memory = HashMap::new();
    let mut mask = Vec::new();
    for instruction in program.lines() {
        if instruction.contains("mask") {
            let m = instruction.split_at(7).1.trim_start_matches('0');
            mask = parse_mask(m);
        } else if instruction.contains("mem") {
            let parts = instruction.split_terminator('=').collect::<Vec<_>>();
            let address = parts[0]
                .trim_matches(|c: char| !c.is_digit(10))
                .parse::<usize>()
                .unwrap();
            let value = parts[1]
                .trim_matches(|c: char| !c.is_digit(10))
                .parse::<usize>()
                .unwrap();

            let possibilities = generate_possibilities(apply_mask(address, &mask));

            for possibility in possibilities {
                let e = memory.entry(possibility).or_default();
                *e = value;
            }
        }
    }

    memory.values().sum()
}

fn main() {
    let content = fs::read_to_string("input").unwrap();

    assert_eq!(part_1(&content), 13496669152158);
    assert_eq!(part_2(&content), 3278997609887);
}
