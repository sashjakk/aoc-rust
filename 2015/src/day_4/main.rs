fn main() {
    let content = std::fs::read_to_string("./src/day_4/input.txt")
        .expect("Unable to read input");
    
    let result1 = find_number(&content, "00000");
    let result2 = find_number(&content, "000000");
    
    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}

fn find_number(key: &str, expected_prefix: &str) -> i32 {
    let mut number = 0;
    
    loop {
        let input = format!("{}{}", key, number);
        let hash = format!("{:x}", md5::compute(input));
        
        if hash.starts_with(expected_prefix) {
            break;
        }

        number += 1;
    }

    return number;
}