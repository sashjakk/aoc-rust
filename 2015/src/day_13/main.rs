use std::collections::HashMap;

use regex::Regex;

fn main() {
    let content = std::fs::read_to_string("./src/day_13/input.txt")
        .expect("Unable to read input");

    let regex = Regex::new(r"(\w+).+\s(\w+)\s(\d+)\s.+\s(\w+)")
        .expect("Unable to init regex");

    let create_entry = |input: &str| {
        regex
            .captures(input)
            .map(|it| {
                let a = &it[1];
                let b = &it[4];
                let unsigned_weight: i32 = (&it[3]).parse().unwrap();

                let weight = match &it[2] {
                    "gain" => unsigned_weight,
                    _ => 0 - unsigned_weight,
                };

                Entry { a: a.to_string(), b: b.to_string(), weight }
            })
    };

    let mut node_map = create_node_map(content.lines(), create_entry);

    let result1 = node_map
        .values()
        .flat_map(|it| get_all_paths_for_node(&node_map, it, Vec::new()))
        .map(|it| get_weight_for_path(&node_map, &it))
        .max()
        .unwrap();

    println!("1st puzzle: {}", result1);

    let node_map2 = {
        let mut host_node = Node { key: "host".to_string(), neighbours: HashMap::new() };
        
        let all_guests = node_map
            .keys()
            .map(|it| it.clone())
            .collect::<Vec<_>>();

        for guest in all_guests {
            node_map
                .get_mut(&guest)
                .unwrap()
                .neighbours.insert("host".to_string(), 0);
            
            host_node.neighbours.insert(guest.clone(), 0);
        }
    
        node_map.insert("host".to_string(), host_node);

        node_map
    };

    let result2 = node_map2
        .values()
        .flat_map(|it| get_all_paths_for_node(&node_map2, it, Vec::new()))
        .map(|it| get_weight_for_path(&node_map2, &it))
        .max()
        .unwrap();

    println!("2nd puzzle: {}", result2);


}

#[derive(Debug)]
struct Entry {
    a: String,
    b: String,
    weight: i32
}

#[derive(Debug)]
struct Node {
    key: String,
    neighbours: HashMap<String, i32>,
}

fn create_node_map<'a>(
    input: impl Iterator<Item = &'a str>,
    entry_factory: impl Fn(&str) -> Option<Entry>,
) -> HashMap<String, Node> {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    let mut insert_with_link = |from: &str, to: &str, weight: i32| {
        match nodes.get_mut(from) {
            Some(existing) => {
                existing.neighbours.insert(to.to_string(), weight);
            },
            None => {
                let node = Node {
                    key: from.to_string(),
                    neighbours: HashMap::from([(to.to_string(), weight)]),
                };
    
                nodes.insert(from.to_string(), node);
            },
        }
    };

    for it in input {
        let entry = entry_factory(&it).unwrap();

        insert_with_link(&entry.a, &entry.b, entry.weight);
    }

    nodes
}

fn get_all_paths_for_node(
    node_map: &HashMap<String, Node>, 
    node: &Node,
    mut path: Vec<String>
) -> Vec<Vec<String>> {
    path.push(node.key.clone());

    let unvisited: Vec<&String> = node.neighbours
        .iter()
        .filter(|it| !path.contains(it.0))
        .map(|it| it.0)
        .collect();
    
    if unvisited.len() == 0 {
        return vec![path];
    }

    unvisited
        .into_iter()
        .flat_map(|it| get_all_paths_for_node(node_map, node_map.get(it).unwrap(), path.clone()))
        .collect()
}

fn get_weight_for_path(
    node_map: &HashMap<String, Node>, 
    path: &Vec<String>
) -> i32 {
    let mut price = 0;
    
    let mut index = 0;

    while index < path.len() {

        let first_key = &path[index % path.len()];
        let second_key = &path[(index + 1) % path.len()];

        let first = node_map.get(first_key).unwrap();
        let second = node_map.get(second_key).unwrap();

        let v1 = first.neighbours.get(second_key).unwrap();
        let v2 = second.neighbours.get(first_key).unwrap();
        
        price += v1 + v2;
        index += 1;
    }

    return price;
}