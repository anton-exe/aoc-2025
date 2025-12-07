pub fn generator(input: &str) -> &str {
    input
}

pub fn part_1(input: &str) -> i64 {
    let mut value = 50;
    let mut count = 0;

    for change in input
        .lines()
        .map(|line| {
            line.replace('L', "-")
                .replace('R', "")
                .parse()
                .unwrap_or_default()
        })
        .collect::<Vec<i64>>()
    {
        value = (value + change).rem_euclid(100);

        if value == 0 {
            count += 1;
        }
    }

    count
}

pub fn part_2(input: &str) -> i64 {
    let mut value = 50;
    let mut count = 0;

    for change in input
        .lines()
        .map(|line| {
            line.replace('L', "-")
                .replace('R', "")
                .parse()
                .unwrap_or_default()
        })
        .collect::<Vec<i64>>()
    {
        let raw_value: i64 = value + change;
        value = raw_value.rem_euclid(100);

        match raw_value.cmp(&0) {
            std::cmp::Ordering::Less => {
                count += (raw_value / 100).abs();
                if raw_value != change {
                    count += 1;
                }
            }
            std::cmp::Ordering::Equal => count += 1,
            std::cmp::Ordering::Greater => count += raw_value / 100,
        }
    }

    count
}
