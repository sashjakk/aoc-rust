use regex::Regex;

fn main() {
    let content = std::fs::read_to_string("./src/day_8/input.txt")
        .expect("Unable to read input");

    let result1 = first_puzzle(content.lines());
    let result2 = second_puzzle(content.lines());
        
    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}

fn first_puzzle<'a>(input: impl Iterator<Item = &'a str>) -> usize {
    let re = Regex::new(r"\\x.{2}").unwrap();

    input
        .map(|it| {
            let literals = it.len();
            let characters = {
                let unescaped = it
                    .replace(r#"\\"#, r#"."#)
                    .replace(r#"\""#, r#"."#);
            
                let unescaped = re.replace_all(&unescaped, ".");

                unescaped[1..unescaped.len() - 1].to_string()
            };

            literals - characters.len()
        })
        .sum()
}

fn second_puzzle<'a>(input: impl Iterator<Item = &'a str>) -> usize {
    input
        .map(|it| {
            let literals = it.len();
            let characters = {
                let escaped = it
                .replace(r#"\"#, r#"\\"#)
                .replace(r#"""#, r#"\""#);

                format!(r#""{}""#, escaped)
            };

            characters.len() - literals
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_puzzle_check() {
        let lines = vec![
            r#""""#, 
            r#""abc""#, 
            r#""aaa\"aaa""#, 
            r#""\x27""#,
        ];

        assert_eq!(12, first_puzzle(lines.into_iter()));
    }

    #[test]
    fn second_puzzle_check() {
        let lines = vec![
            r#""""#, 
            r#""abc""#, 
            r#""aaa\"aaa""#, 
            r#""\x27""#,
        ];

        assert_eq!(19, second_puzzle(lines.into_iter()));
    }
}