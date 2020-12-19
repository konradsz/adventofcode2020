use std::fs;

fn evaluate(expression: &str) -> u64 {
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

        let result = evaluate_to_string(&part);

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

    evaluate_flat(&expression)
}

fn evaluate_flat(expression: &str) -> u64 {
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

fn evaluate_to_string(expression: &str) -> String {
    evaluate_flat(expression).to_string()
}

fn part_1(expressions: &str) -> u64 {
    expressions.lines().map(|expr| evaluate(expr)).sum()
}

fn main() {
    let expressions = fs::read_to_string("input").unwrap();

    println!("{}", part_1(&expressions));
}
