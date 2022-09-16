fn main() {
    let content = std::fs::read_to_string("./src/day_15/input.txt")
        .expect("Unable to read input");

    let ingredients = content
        .lines()
        .filter_map(Ingredient::parse)
        .collect::<Vec<_>>();

    let result1 = brute_force(4, 100)
        .filter(|it| it.iter().sum::<i32>() == 100)
        .map(|it| cookie_score(&ingredients, &it))
        .max()
        .unwrap();

    let result2 = brute_force(4, 100)
        .filter(|it| it.iter().sum::<i32>() == 100 && cookie_calories(&ingredients, &it) == 500)
        .map(|it| cookie_score(&ingredients, &it))
        .max()
        .unwrap();

    println!("1st puzzle: {}", result1);
    println!("2nd puzzle: {}", result2);
}

fn cookie_score(ingredients: &Vec<Ingredient>, amounts: &Vec<i32>) -> i32 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;

    for (index, ingridient) in ingredients.iter().enumerate() {
        capacity += ingridient.capacity * amounts[index];
        durability += ingridient.durability * amounts[index];
        flavor += ingridient.flavor * amounts[index];
        texture += ingridient.texture * amounts[index];
    }

    std::cmp::max(0, capacity) * 
    std::cmp::max(0, durability) * 
    std::cmp::max(0, flavor) * 
    std::cmp::max(0, texture)
}

fn cookie_calories(ingredients: &Vec<Ingredient>, amounts: &Vec<i32>) -> i32 {
    let mut calories = 0;

    for (index, ingridient) in ingredients.iter().enumerate() {
        calories += ingridient.calories * amounts[index];
    }

    std::cmp::max(0, calories)
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn parse(input: &str) -> Option<Ingredient> {
        let name_and_props = input.split(":").collect::<Vec<_>>();
        let props = name_and_props.last()?.split(",").collect::<Vec<_>>();

        let ingridient = Ingredient {
            capacity: props[0].split(" ").last()?.parse::<i32>().ok()?,
            durability: props[1].split(" ").last()?.parse::<i32>().ok()?,
            flavor: props[2].split(" ").last()?.parse::<i32>().ok()?,
            texture: props[3].split(" ").last()?.parse::<i32>().ok()?,
            calories: props[4].split(" ").last()?.parse::<i32>().ok()?,
        };

        Some(ingridient)
    }
}

fn brute_force(positions: usize, max_value: i32) -> impl Iterator<Item = Vec<i32>> {
    let mut values = vec![0; positions];
    values[positions - 1] = -1;

    let instance = Brute { values, max: max_value };

    return instance.into_iter();
}

struct Brute {
    values: Vec<i32>,
    max: i32,
}

impl Iterator for Brute {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.iter().all(|it| *it == self.max) {
            return None;
        }

        let last = self.values.len() - 1;

        if self.values[last] < self.max {
            self.values[last] = self.values[last] + 1;
            return Some(self.values.clone());
        }

        if self.values[last] == self.max {
            let mut i = last;

            while i >= 1 && self.values[i] == self.max {
                self.values[i] = 0;
                i -= 1;
            }

            self.values[i] = self.values[i] + 1;

            return Some(self.values.clone());
        }

        None
    }
}