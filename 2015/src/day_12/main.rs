use serde_json::{self, Value};

fn main() {
    let content = std::fs::read_to_string("./src/day_12/input.txt")
        .expect("Unable to read input");

    let json: Value = serde_json::from_str(&content).unwrap();

    let result1 = first_puzzle(0.0, &json);
    let result2 = second_puzzle(0.0, &json);

    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}

fn first_puzzle(acc: f64, json: &Value) -> f64 {
    match json {
        Value::Number(it) => acc + it.as_f64().unwrap(),
        Value::Array(it) => acc + it.into_iter().fold(0.0, |a, b| first_puzzle(a, b)),
        Value::Object(it) => acc + it.into_iter().fold(0.0, |a, (_, b)| first_puzzle(a, b)),
        _ => acc
    }
}

fn second_puzzle(acc: f64, json: &Value) -> f64 {
    match json {
        Value::Number(it) => acc + it.as_f64().unwrap(),
        Value::Array(it) => acc + it.into_iter().fold(0.0, |a, b| second_puzzle(a, b)),
        Value::Object(it) => {
            let found = it
                .values()
                .filter_map(|value| value.as_str())
                .find(|value| *value == "red");

            match found {
                Some(_) => acc,
                None => acc + it.into_iter().fold(0.0, |a, (_, b)| second_puzzle(a, b))
            }
        }
        _ => acc
    }
}