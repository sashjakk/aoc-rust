fn main() {
    let content = std::fs::read_to_string("./src/day_1/input.txt")
        .expect("Unable to read input");

    let result1 = first_puzzle(&content);
    let result2 = second_puzzle(&content);

    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}

fn first_puzzle(content: &str) -> i32 {
    return content
        .chars()
        .fold(0, |acc, it|
            match it {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc,
            }
        );
}

fn second_puzzle(content: &str) -> usize {
    let mut current_floor = 0;
    
    for (index, it) in content.chars().enumerate() {
        if it == '(' { current_floor += 1 } else { current_floor -= 1 }

        if current_floor == -1 {
            return index + 1;
        }
    }

    return 0;
}

