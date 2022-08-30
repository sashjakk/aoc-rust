fn main() {
    let content = std::fs::read_to_string("./src/day_11/input.txt")
        .expect("Unable to read input");

    let result1 = derive_next_password(&content);
    let result2 = derive_next_password(&result1);

    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}

fn increment(input: &str) -> String {
    let mut password = Vec::new();
    let mut propagate = true;

    for c in input.chars().rev() {
        let code = c as u8;

        if propagate {
            if code + 1 > ('z' as u8) { 
                propagate = true;
                password.insert(0, 'a');
            } else {
                propagate = false;
                password.insert(0, (code + 1) as char);
            }; 
        } else {
            password.insert(0, c);
        }
        
    }

    String::from_iter(password)
}

fn is_valid(input: &str) -> bool {
    let mut straight_window = Vec::new();
    let mut straight_found = false;

    let mut pair_window = Vec::new();
    let mut pairs_found = 0;
    
    for c in input.chars() {
        // confusing characters
        match c {
            'i' | 'o' | 'l' => return false,
            _ => {},
        }

        // increasing straight
        straight_window.push(c as i32);
        if straight_window.len() > 3 {
            straight_window.remove(0);
        }

        if straight_window.len() == 3 {
            if (straight_window[2] - straight_window[1]) == 1 && (straight_window[1] - straight_window[0]) == 1 {
                straight_found = true;
            }
        }
        

        // pairs
        pair_window.push(c as i32);
        if pair_window.len() > 2 {
            pair_window.remove(0);
        }

        if pair_window.len() == 2 {
            if pair_window[0] == pair_window[1] {
                pairs_found = pairs_found + 1;
                pair_window.clear();
            } 
        }
    }

    return straight_found && pairs_found >= 2;
}

fn derive_next_password(input: &str) -> String {
    let mut next = input.to_string();
    
    loop {
        next = increment(&next);
        
        if is_valid(&next) { 
            break; 
        }
    }

    next
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn validates_password_requirements() {
        let table = HashMap::from([
            ("hijklmmn", false),
            ("abbceffg", false),
            ("abbcegjk", false),
            ("abcdeggg", false),
            ("ghijklmn", false),
            ("abcdffaa", true),
        ]); 

        for (pass, result) in table.iter() {
            assert_eq!(is_valid(pass), *result);
        }
    }

    #[test]
    fn validates_increment() {
        assert_eq!(increment("xx"), "xy");
        assert_eq!(increment("xxz"), "xya");
    }
}