use regex::Regex;
use std::fs;

#[derive(Clone, Debug)]
struct Bag {
    name: String,
    amount: i32,
    contains: Vec<Bag>,
}

impl Bag {
    fn can_hold(&self, bag: &Bag) -> bool {
        for inner_bag in &self.contains {
            if inner_bag.name == bag.name {
                return true;
            }
        }
        false
    }

    fn inner_bags(&self) -> Vec<String> {
        let mut inner_bags = Vec::new();
        for bag in &self.contains {
            for _ in 0..bag.amount {
                inner_bags.push(bag.name.clone());
            }
        }
        inner_bags
    }
}

fn parse_input() -> Vec<Bag> {
    let contents = fs::read_to_string("input").expect("Unable to read input file");
    let raw_bags = contents.lines();
    let mut bags = Vec::new();

    let outer_re = Regex::new(r"(.*)bags contain(.*)").unwrap();
    let inner_re = Regex::new(r"(\d*)(.*)bag").unwrap();
    for raw_bag in raw_bags {
        let cap = outer_re.captures(raw_bag).unwrap();
        if cap.len() != 3 {
            panic!("Capture group less than 3?");
        }

        let mut bag = Bag {
            name: cap[1].trim().to_string(),
            amount: 0,
            contains: Vec::new(),
        };

        if !raw_bag.contains("no other bags") {
            let raw_contains = cap[2].split(",").map(str::trim);
            for raw_inner_bag in raw_contains {
                let inner_cap = inner_re.captures(raw_inner_bag).unwrap();
                let inner_bag = Bag {
                    name: inner_cap[2].trim().to_string(),
                    amount: inner_cap[1].parse().unwrap(),
                    contains: Vec::new(),
                };
                bag.contains.push(inner_bag);
            }
        }

        bags.push(bag);
    }

    bags
}

fn get_bag(bags: &Vec<Bag>, name: String) -> Option<&Bag> {
    for bag in bags {
        if name == bag.name {
            return Some(bag);
        }
    }

    None
}

fn can_contain(bags: &Vec<Bag>, bag_to_test: &Bag) -> Vec<String> {
    let mut matched_bags = Vec::new();

    for bag in bags {
        if bag.can_hold(bag_to_test) {
            matched_bags.push(bag.name.clone());
            matched_bags.append(&mut can_contain(&bags, bag));
        }
    }

    matched_bags
}

fn nested_count(bags: &Vec<Bag>, bag: &Bag) -> i32 {
    let mut bag_count = 0;
    if bag.contains.len() == 0 {
        return bag_count;
    }

    let inner_bags = bag.inner_bags();
    bag_count += inner_bags.len() as i32;
    for inner_bag_name in inner_bags {
        if let Some(inner_bag) = get_bag(bags, inner_bag_name) {
            bag_count += nested_count(bags, inner_bag);
        }
    }

    bag_count
}

fn main() {
    let bags = parse_input();
    let bag_to_test = Bag {
        name: "shiny gold".to_string(),
        amount: 0,
        contains: vec![],
    };
    let mut matches = can_contain(&bags, &bag_to_test);
    matches.sort();
    matches.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    println!("{} bags can hold a {} bag", matches.len(), bag_to_test.name);

    let mut total_bags = 0;
    if let Some(bag) = get_bag(&bags, "shiny gold".to_string()) {
        total_bags += nested_count(&bags, &bag);
    }
    println!("Total nested bags inside shiny gold bag: {}", total_bags);
}
