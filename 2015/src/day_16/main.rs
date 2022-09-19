use core::str;
use std::{collections::HashMap, iter::FromIterator};

fn main() {
    let content = std::fs::read_to_string("./src/day_16/input.txt")
        .expect("Unable to read input");

    let aunts = content
        .lines()
        .filter_map(Aunt::parse)
        .collect::<Vec<_>>();

    let known_compounds = Aunt {
        id: 0,
        compounds: HashMap::from_iter([
            ("children".to_string(), 3),
            ("cats".to_string(), 7),
            ("samoyeds".to_string(), 2),
            ("pomeranians".to_string(), 3),
            ("akitas".to_string(), 0),
            ("vizslas".to_string(), 0),
            ("goldfish".to_string(), 5),
            ("trees".to_string(), 3),
            ("cars".to_string(), 2),
            ("perfumes".to_string(), 1),
        ]),    
    };

    let (result1, _) = aunts
        .iter()
        .map(|it| (it, it.match_score(&known_compounds)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    let (result2, _) = aunts
        .iter()
        .map(|it| (it, it.match_score_with_ranges(&known_compounds)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    println!("1st puzzle: {}", result1.id);
    println!("2nd puzzle: {}", result2.id);
}

trait DropLastChars {
    type Item;

    fn drop(&self, amount: i32) -> Self::Item;
}

impl<'a> DropLastChars for &'a str {
    type Item = &'a str;

    fn drop(&self, amount: i32) -> &'a str {
        let mut chars = self.chars();
        
        for _ in 0..amount { 
            chars.next_back(); 
        }
        
        chars.as_str()
    }
}

#[derive(Debug)]
struct Aunt {
    id: u16,
    compounds: HashMap<String, u8>,
}

impl Aunt {
    fn parse(input: &str) -> Option<Aunt> {
        let props = input
            .split(" ")
            .collect::<Vec<_>>();

        let mut compounds: HashMap<String, u8> = HashMap::new();

        let mut index = 2;
        
        while index < (props.len() - 1) {
            let key = props[index].drop(1);
            let value = if props[index + 1].ends_with(",") { props[index + 1].drop(1) } else { props[index + 1] };

            compounds.insert(key.to_string(), value.parse::<u8>().ok()?);
            index += 2;
        }

        Some(Aunt { 
            id: props[1].drop(1).parse().ok()?, 
            compounds 
        })
    }

    fn match_score(&self, other: &Aunt) -> u8 {
        self.compounds
            .iter()
            .map(|(key, value)| {
                if let Some(other_value) = other.compounds.get(key) { 
                    if *other_value == *value {
                        return 1;
                    }
                }

                return 0;
            })
            .sum()
    }

    fn match_score_with_ranges(&self, other: &Aunt) -> u8 {
        self.compounds
            .iter()
            .map(|(key, value)| {
                if let Some(other_value) = other.compounds.get(key) { 
                    return match key.as_str() {
                        "cats" | "trees" => if *value > *other_value { 1 } else { 0 }, 
                        "pomeranians" | "goldfish" => if *value < *other_value { 1 } else { 0 },
                        _ => if *other_value == *value { 1 } else { 0 },
                    }
                }

                return 0;
            })
            .sum()
    }
}