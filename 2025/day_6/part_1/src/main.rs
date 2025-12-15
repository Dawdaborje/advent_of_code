use std::fs;

fn main() {
    let input = fs::read_to_string("src/puzzle_input.txt").unwrap();

    let mut rows: Vec<Vec<String>> = vec![];

    for line in input.lines() {
        if !line.trim().is_empty() {
            let tokens = line
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            rows.push(tokens);
        }
    }

    let operator_row_index = rows.len() - 1;
    let col_count = rows.iter().map(|r| r.len()).max().unwrap();

    let mut total: u128 = 0;

    for col in 0..col_count {
        let mut values: Vec<u128> = vec![];

        for r in 0..operator_row_index {
            if col < rows[r].len() {
                if let Ok(n) = rows[r][col].parse::<u128>() {
                    values.push(n);
                }
            }
        }

        let op = if col < rows[operator_row_index].len() {
            rows[operator_row_index][col].as_str()
        } else {
            continue;
        };

        let result = match op {
            "+" => values.iter().copied().sum(),
            "*" => values.iter().copied().fold(1u128, |acc, x| acc * x),
            _ => continue,
        };

        total += result;
    }

    println!("{}", total);
}
