use std::collections::HashMap; 

fn main() {
    let content = std::fs::read_to_string("./src/day_3/input.txt")
        .expect("Unable to read input");

    let position1 = (0, 0);
    let mut houses1 = HashMap::from([(position1, 1)]);
    
    deliver_gifts(position1, &mut houses1, content.chars());
    let result1 = houses1.len();
    
    println!("1st puzzle: {}", result1);
    
    
    let position2 = (0, 0);
    let mut houses2 = HashMap::from([(position2, 1)]);

    let santa_path = content
        .char_indices()
        .filter_map(|(index, it)| if index % 2 == 0 { Some(it) } else { None } );

    let robo_path = content
        .char_indices()
        .filter_map(|(index, it)| if index % 2 != 0 { Some(it ) } else { None } );

    deliver_gifts(position2, &mut houses2, santa_path);
    deliver_gifts(position2, &mut houses2, robo_path);
    let result2 = houses2.len();

    println!("2nd puzzle: {}", result2);
}

fn deliver_gifts(
    position: (i32, i32),
    houses: &mut HashMap<(i32, i32), i32>,
    directions: impl Iterator<Item = char>,
) {
    let mut curr = position;

    for it in directions {
        let (x, y) = match it {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => (0, 0),
        };
        curr = (curr.0 + x, curr.1 + y);

        let value = if let Some(it) = houses.get(&curr) { it + 1 } else { 1 };
        houses.insert(curr, value);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::deliver_gifts;
//     use std::collections::HashMap;

//     #[test]
//     fn delivers_to_two_houses() {
//         let position = (0, 0);
//         let mut houses = HashMap::from([((0, 0), 1)]);

//         deliver_gifts(position, &mut houses, ">");
//         assert_eq!(houses.len(), 2);
//     }

//     #[test]
//     fn delivers_to_four_houses() {
//         let position = (0, 0);
//         let mut houses = HashMap::from([((0, 0), 1)]);

//         deliver_gifts(position, &mut houses, "^>v<");
//         assert_eq!(houses.len(), 4);
//     }

//     #[test]
//     fn delivers_to_two_houses_again() {
//         let position = (0, 0);
//         let mut houses = HashMap::from([((0, 0), 1)]);

//         deliver_gifts(position, &mut houses, "^v^v^v^v^v");
//         assert_eq!(houses.len(), 2);
//     }
// }