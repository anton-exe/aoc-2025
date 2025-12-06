pub fn generator(input: &str) -> &str {
    input
}

pub fn part_1(input: &str) -> u64 {
    input
        .split_once('\n')
        .unwrap_or_default()
        .0
        .split_whitespace()
        .enumerate()
        .map(|(idx, _)| {
            let mut operands = Vec::<u64>::new();
            let mut operation = " ";
            let () = input
                .lines()
                .map(|line| {
                    let cell = line.split_whitespace().nth(idx).unwrap_or_default();
                    if cell == "+" || cell == "*" {
                        operation = cell;
                    } else {
                        operands.push(cell.parse().unwrap_or_default());
                    }
                })
                .collect::<()>();

            match operation {
                "+" => operands.iter().sum(),
                "*" => {
                    let mut product = 1;
                    let () = operands.iter().map(|val| product *= val).collect::<()>();
                    product
                }
                &_ => 0,
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> u64 {
    let mut total = 0;
    let mut operands = Vec::<u64>::new();
    let mut operation = ' ';

    for (idx, c) in input.lines().last().unwrap_or_default().chars().enumerate() {
        if idx == 0 {
            operation = c;
        } else if c == '+' || c == '*' {
            total += match operation {
                '+' => operands.iter().sum(),
                '*' => {
                    let mut product = 1;
                    let () = operands.iter().map(|val| product *= val).collect::<()>();
                    product
                }
                _ => 0,
            };

            operation = c;
            operands = Vec::<u64>::new();
        } else {
            let n = input
                .lines()
                .map(|line| line.chars().nth(idx - 1).unwrap_or_default())
                .collect::<String>()
                .replace(['+', '*', ' '], "");
            if !n.is_empty() {
                operands.push(n.parse::<u64>().unwrap_or_default());
            }
        }
    }
    let n = input
        .lines()
        .map(|line| line.chars().last().unwrap_or_default())
        .collect::<String>()
        .replace(['+', '*', ' '], "");
    if !n.is_empty() {
        operands.push(n.parse::<u64>().unwrap_or_default());
    }
    total += match operation {
        '+' => operands.iter().sum(),
        '*' => {
            let mut product = 1;
            let () = operands.iter().map(|val| product *= val).collect::<()>();
            product
        }
        _ => 0,
    };

    total
}
