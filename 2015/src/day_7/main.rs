use std::{collections::HashMap};

fn main() {
    let content = std::fs::read_to_string("./src/day_7/input.txt")
        .expect("Unable to read input");

    let result1 = {
        let mut lines: Vec<_> = content.lines().collect();
        let mut wires: HashMap<String, u16> = HashMap::new();
        
        loop {
            let next: Vec<_> = lines
                .into_iter()
                .filter(|it| !dispatch(it, &mut wires))
                .collect();
    
            if next.len() == 0 { break; }
            
            lines = next;
        }
    
        wires.get("a").unwrap().clone()
    };

    let result2 = {
        let mut lines: Vec<_> = content
            .lines()
            .map(|it| if it.ends_with("-> b") { format!("{} -> b", result1) } else { format!("{}", it) })
            .collect();
        
        let mut wires: HashMap<String, u16> = HashMap::new();
        
        loop {
            let next: Vec<_> = lines
                .into_iter()
                .filter(|it| !dispatch(it, &mut wires))
                .collect();
    
            if next.len() == 0 { break; }
            
            lines = next;
        }
    
        wires.get("a").unwrap().clone()
    };

    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}

fn get_value(value_or_key: &str, wires: &HashMap<String, u16>) -> Option<u16> {
    value_or_key
        .parse()
        .ok()
        .or_else(|| wires.get(value_or_key).cloned())
}

fn process(
    input: &str,
    wires: &mut HashMap<String, u16>, 
    action: &str, 
    value: fn(u16, u16) -> u16,
) -> bool {
    let parts: Vec<&str> = input.split(" -> ").collect();
    let args: Vec<&str> = parts[0].split(action).collect();

    let first = get_value(args[0], wires);
    let second = get_value(args[1], wires);

    if let (Some(a), Some(b)) = (first, second) {
        wires.insert(parts[1].to_string(), value(a, b));
        return true;
    } else {
        return false;
    }
}

fn dispatch(input: &str, wires: &mut HashMap<String, u16>) -> bool {
    if input.contains("AND") {
        return process(input, wires, " AND ", |a, b| a & b);
    }

    if input.contains("OR") {
        return process(input, wires, " OR ", |a, b| a | b);
    }

    if input.contains("LSHIFT") {
        return process(input, wires, " LSHIFT ", |a, b| a << b);
    }

    if input.contains("RSHIFT") {
        return process(input, wires, " RSHIFT ", |a, b| a >> b);
    }

    if input.contains("NOT") {
        let parts: Vec<&str> = input.split(" -> ").collect();
        let args: Vec<&str> = parts[0].split("NOT ").collect();

        let second = get_value(args[1], wires);
    
        if let Some(a)= second {
            wires.insert(parts[1].to_string(), !a);
            return true;
        } else {
            return false;
        }
    }
    
    let args: Vec<&str> = input.split(" -> ").collect();

    let first = get_value(args[0], wires);

    if let Some(value) = first {
        wires.insert(args[1].to_string(), value);
        return true;
    } else {
        return false;
    }

}
