use std::collections::HashMap;

use regex::Regex;

fn main() {
    let content = std::fs::read_to_string("./src/day_14/input.txt")
        .expect("Unable to read input");

    let regex = Regex::new(r"(\w+)\s.+\s(\d+)\skm/s.+for\s(\d+)\sseconds,.+for\s(\d+)\sseconds")
        .expect("Unable to init regex");

    let deers = content
        .lines()
        .filter_map(|it| Reindeer::parse(it, &regex))
        .collect::<Vec<_>>();

    let result1 = deers
        .iter()
        .map(|it| it.distance(2503))
        .max()
        .unwrap();
    
    println!("1st puzzle: {}", result1);

    let result2 = {
        let mut points = HashMap::new();
    
        for i in 1..=2503 {
            let best = deers
                .iter()
                .map(|it| (it.name.clone(), it.distance(i)))
                .max_by(|(_, a), (_, b)| a.cmp(&b))
                .unwrap();
            
            match points.get(&best.0).or(Some(&0)) {
                Some(current) => { points.insert(best.0.clone(), current + 1); },
                _ => {},
            }
        }

        points.values().max().unwrap().clone()
    };

    println!("2nd puzzle: {}", result2);
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u16,
    flying_time: u16,
    rest_time: u16,
}

impl Reindeer {
    fn parse(input: &str, regex: &Regex) -> Option<Reindeer> {
        let captures = regex.captures(input)?;
        
        let reindeer = Reindeer {
            name: captures[1].to_string(),
            speed: captures[2].parse().ok()?,
            flying_time: captures[3].parse().ok()?,
            rest_time: captures[4].parse().ok()?,
        };

        Some(reindeer)
    }

    fn distance(&self, seconds: u16) -> u16 {
        let scale = self.flying_time + self.rest_time;

        let multiplier = (seconds / scale) + if (seconds % scale) > self.flying_time { 1 } else { 0 };
        let elapsed = self.flying_time * multiplier;

        if (seconds % scale) <= self.flying_time {
            self.speed * (seconds % scale) + self.speed * elapsed
        } else {
            self.speed * elapsed
        }
    }
}