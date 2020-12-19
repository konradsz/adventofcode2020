use std::fs;

fn evaluate(expression: &str, evaluate_by_rules: fn(&str) -> u64) -> u64 {
    let mut expression = String::from(expression);
    while let Some(open_bracket) = expression.bytes().rposition(|c| c == b'(') {
        let closed_bracket = (&expression.as_bytes()[open_bracket..])
            .iter()
            .position(|&c| c == b')')
            .unwrap();

        let part = String::from_utf8(
            (&expression.as_bytes()[open_bracket + 1..open_bracket + closed_bracket]).into(),
        )
        .unwrap();

        let result = evaluate_to_string(&part, evaluate_by_rules);

        let mut reduced =
            String::from_utf8((&expression.as_bytes()[0..open_bracket]).into()).unwrap();
        reduced.push_str(&result);
        reduced.push_str(
            &String::from_utf8(
                (&expression.as_bytes()[open_bracket + closed_bracket + 1..]).into(),
            )
            .unwrap(),
        );
        expression = reduced;
    }

    evaluate_by_rules(&expression)
}

fn evaluate_flat_basic(expression: &str) -> u64 {
    let mut parts = expression.split_ascii_whitespace();

    let mut result = 0;
    while let Some(next) = parts.next() {
        if let Ok(number) = next.parse::<u64>() {
            result = number;
        } else if next == "+" {
            let rhs = parts.next().unwrap().parse::<u64>().unwrap();
            result += rhs;
        } else if next == "*" {
            let rhs = parts.next().unwrap().parse::<u64>().unwrap();
            result *= rhs;
        }
    }

    result
}

fn evaluate_flat_advanced(expression: &str) -> u64 {
    let additions = expression.split(" * ");
    additions
        .map(|addition| evaluate_flat_basic(addition))
        .product()
}

fn evaluate_to_string(expression: &str, evaluate_by_rules: fn(&str) -> u64) -> String {
    evaluate_by_rules(expression).to_string()
}

fn part_1(expressions: &str) -> u64 {
    expressions
        .lines()
        .map(|expr| evaluate(expr, evaluate_flat_basic))
        .sum()
}

fn part_2(expressions: &str) -> u64 {
    expressions
        .lines()
        .map(|expr| evaluate(expr, evaluate_flat_advanced))
        .sum()
}

fn main() {
    let expressions = fs::read_to_string("input").unwrap();

    assert_eq!(part_1(&expressions), 11_004_703_763_391);
    assert_eq!(part_2(&expressions), 290_726_428_573_651)
}
