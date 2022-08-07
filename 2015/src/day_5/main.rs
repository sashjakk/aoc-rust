use std::collections::HashMap;

fn main() {
    let content = std::fs::read_to_string("./src/day_5/input.txt")
        .expect("Unable to read input");

    let result1 = content
        .lines()
        .filter(|it| { first_puzzle(*it) })
        .count();

    let result2 = content
        .lines()
        .filter(|it| { second_puzzle(*it) })
        .count();
    
    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}

fn first_puzzle(input: &str) -> bool {
    let mut vowels = 0;
    let mut has_duplicate = false;

    for i in 0..input.len() {
        let curr = input.chars().nth(i).unwrap();
        
        if "aeiou".chars().any(|c| c == curr) { vowels += 1; }

        if let Some(next) = input.chars().nth(i + 1) {
            match (curr, next) {
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => return false,
                _ => {}
            };

            if has_duplicate { continue };
            if curr == next { has_duplicate = true; }
        }
    }

    return has_duplicate && vowels >= 3;
}

fn second_puzzle(input: &str) -> bool {
    let mut pairs: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
    let mut has_same_neighbours = false;

    for i in 0..input.len() {
        if let (Some(curr), Some(next)) = (input.chars().nth(i), input.chars().nth(i + 1)) {
            let key = String::from_iter([curr, next]);
            let pair = (i, (i + 1));
            
            if let Some(existing) = pairs.get_mut(&key) {
                existing.push(pair);
            } else {
                pairs.insert(key, vec![pair]);
            }
        }
    }

    let has_duplicates = pairs
        .values()
        .filter(|it| it.len() >= 2)
        .count() > 0;

    if !has_duplicates { return false; }

    let has_overlaps = pairs
        .values()
        .filter(|&it| it.len() >= 2)
        .filter(|&it| {
            for i in 0..it.len() {
                if let (Some((_, end)), Some((start,_))) = (it.get(i), it.get(i + 1)) {
                    if end >= start { return true }
                }
            }

            false
        })
        .count() > 0;

    if has_overlaps { return false };

    for i in 0..input.len() {
        if let (Some(prev), Some(next)) = (input.chars().nth(i), input.chars().nth(i + 2)) {
            if prev == next { has_same_neighbours = true };
        }
    }

    if !has_same_neighbours { return false; }

    return true;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn nice_check() {
        let table = HashMap::from([
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ]);

        for (input, expected) in table {
            assert_eq!(first_puzzle(input), expected);
        }
    }

    #[test]
    fn second_puzzle_check() {
        let table = HashMap::from([
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
            ("aaa", false),
        ]);

        for (input, expected) in table {
            assert_eq!(second_puzzle(input), expected);
        }
    }
}