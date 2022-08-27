fn main() {
    let content = std::fs::read_to_string("./src/day_10/input.txt")
        .expect("Unable to read input");

    let result1 = (0..40)
        .fold(content.clone(), |acc, _| { look_and_say(&acc).unwrap() })
        .len();

    let result2 = (0..50)
        .fold(content.clone(), |acc, _| { look_and_say(&acc).unwrap() })
        .len();

    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}


fn look_and_say(input: &str) -> Option<String> {
    let mut current_char = input.chars().nth(0)?;
    let mut count = 0;
    
    let mut result = Vec::new();
    
    for char in input.chars() {
        if current_char == char {
            count = count + 1;
        } else {
            result.push(std::char::from_digit(count, 10)?);   
            result.push(current_char);

            current_char = char;
            count = 1;
        }
    }

    result.push(std::char::from_digit(count, 10)?);   
    result.push(current_char);

    Some(String::from_iter(result))
}