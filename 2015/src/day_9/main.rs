use std::{
    collections::{HashMap},
    fmt::Debug, usize,
};

fn main() {
    let content = std::fs::read_to_string("./src/day_9/input.txt").expect("Unable to read input");

    let node_map = create_node_map(content.lines());
    
    let result1 = node_map
        .values()
        .flat_map(|it| get_all_paths_for_node(&node_map, it, Vec::new()))
        .map(|it| get_weight_for_path(&node_map, &it))
        .min()
        .unwrap();

    let result2 = node_map
        .iter()
        .flat_map(|(_, node)| get_all_paths_for_node(&node_map, node, Vec::new()))
        .map(|it| get_weight_for_path(&node_map, &it))
        .max()
        .unwrap();

    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}

#[derive(Debug)]
struct Node {
    key: String,
    neighbours: HashMap<String, usize>,
}

fn create_node_map<'a>(input: impl Iterator<Item = &'a str>) -> HashMap<String, Node> {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    let mut insert_with_link = |from: &str, to: &str, weight: usize| {
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
        let points_and_distance = it.split(" = ").collect::<Vec<&str>>();
        
        let points = points_and_distance[0].split(" to ").collect::<Vec<&str>>();
        let distance = points_and_distance[1].parse::<usize>().unwrap();

        insert_with_link(points[0], points[1], distance);
        insert_with_link(points[1], points[0], distance);
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
        return vec![path.clone()];
    }

    unvisited
        .into_iter()
        .flat_map(|it| get_all_paths_for_node(node_map, node_map.get(it).unwrap(), path.clone()))
        .collect()
}

fn get_weight_for_path(node_map: &HashMap<String, Node>, path: &Vec<String>) -> usize {
    let mut price = 0;
    
    for index in 0..path.len() - 1 {
        let first_key = path.get(index).unwrap();
        let second_key = path.get(index + 1).unwrap();

        let first = node_map.get(first_key).unwrap();
        let value = first.neighbours.get(second_key).unwrap();

        price = price + value;
    }

    price
}