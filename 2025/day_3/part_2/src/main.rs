use std::fs;

fn max_12_digit_number(line: &str) -> String {
    let digits: Vec<char> = line.chars().collect();
    let n = digits.len();
    let remove = n - 12;

    let mut stack: Vec<char> = Vec::with_capacity(n);
    let mut to_remove = remove;

    for &d in &digits {
        while !stack.is_empty() && to_remove > 0 && *stack.last().unwrap() < d {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(d);
    }

    while to_remove > 0 {
        stack.pop();
        to_remove -= 1;
    }

    stack.into_iter().take(12).collect()
}

fn main() {
    let file = fs::read_to_string("src/puzzle_input.txt").unwrap();
    let mut total: u128 = 0;

    for line in file.lines() {
        let result = max_12_digit_number(line.trim());
        let value: u128 = result.parse().unwrap();
        total += value;
    }

    println!("Answer: {}", total);
}
