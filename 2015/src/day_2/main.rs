fn main() {
    let content = std::fs::read_to_string("./src/day_2/input.txt")
        .expect("Unable to read input");

    let result1 = first_puzzle(&content);
    let result2 = second_puzzle(&content);
    
    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}

fn first_puzzle(input: &str) -> i32 {
    return input
        .lines()
        .filter_map(|it| {
            let sides = it
                .split("x")
                .filter_map(|it| it.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            let l = sides[0];
            let h = sides[1];
            let w = sides[2];

            let surfaces = [l * h, l * w, w * h];
            
            let all_surface_area: i32 = surfaces
                .iter()
                .map(|it| it * 2)
                .sum();

            return surfaces
                .iter()
                .min()
                .map(|it| all_surface_area + it);
        })
        .sum();
}

fn second_puzzle(input: &str) -> i32 {
    return input
        .lines()
        .filter_map(|it| {
            let sides = it
                .split("x")
                .filter_map(|it| it.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            let l = sides[0];
            let h = sides[1];
            let w = sides[2];

            let perimeters = [2 * (l + h), 2 * (l + w), 2 * (w + h)];

            let bow = l * h * w;
            let ribbon = perimeters.iter().min();
            
            return ribbon.map(|it| bow + it);
        })
        .sum();
}