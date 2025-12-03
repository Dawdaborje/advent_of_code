use std::fs;

fn is_invalid_id(number: u64) -> bool {
    let s = number.to_string();
    let len = s.len();

    for slen in 1..=len / 2 {
        if len % slen != 0 {
            continue;
        }
        let repeat_count = len / slen;
        let sub = &s[0..slen];
        if sub.repeat(repeat_count) == s {
            return true;
        }
    }

    return false;
}

fn main() {
    let input = fs::read_to_string("/home/borje/Documents/CODING/Personal/personal/advent_of_code/2025/day_2/part_2/src/puzzle_input.txt").unwrap();
    let mut invalid_ids: Vec<u64> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let range = line.split('-').collect::<Vec<_>>();
        let range_start: u64 = range[0].parse().expect("Failed to parse value");
        let range_end: u64 = range[1].parse().expect("Failed to parse value");

        for n in range_start - 1..range_end + 1 {
            if is_invalid_id(n) {
                invalid_ids.push(n)
            };
        }
    }

    let sum: u64 = invalid_ids.iter().sum();
    println!("The sum of: {:?}", sum);
}
