use std::fs;

fn get_all_posible_compinations(num_str: &str) -> Vec<u32> {
    let mut result = Vec::new();
    let chars: Vec<char> = num_str.chars().collect();

    for i in 0..chars.len() {
        for j in i + 1..chars.len() {
            let value = format!("{}{}", chars[i], chars[j]).parse::<u32>().unwrap();
            result.push(value);
        }
    }

    result
}

fn find_high_combination(number: &str) -> u32 {
    let all = get_all_posible_compinations(number);
    *all.iter().max().unwrap()
}

fn main() {
    let file = fs::read_to_string("src/puzzle_input.txt").unwrap();
    let mut total = 0u64;

    for line in file.lines() {
        let high = find_high_combination(line.trim());
        total += high as u64;
    }

    println!("Answer: {}", total);
}
