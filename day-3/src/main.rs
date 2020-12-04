use std::fs;

#[derive(Debug)]
struct Ruleset {
    x_delta: usize,
    y_delta: usize,
}

fn parse_input() -> Vec<Vec<i32>> {
    /* Turn the input into a matrix where 0 is no tree and 1 is tree
     * e.g
     * [[ 0, 0, 0, 1 ]
     *  [ 1, 0, 0, 0 ]
     *  [ 1, 1, 1, 0 ]]
     */
    let mut input: Vec<Vec<i32>> = Vec::new();
    let contents = fs::read_to_string("input").expect("Unable to read input file");
    let lines = contents.lines();

    for (i, line) in lines.enumerate() {
        input.push(Vec::new());
        for character in line.chars() {
            if character == '#' {
                input[i].push(1);
            } else {
                input[i].push(0);
            }
        }
    }

    input
}

// Hardcoded first example; Refactored into traverse_slope
fn part_one(input: &Vec<Vec<i32>>) -> i32 {
    // Rule: Right 3, Down 1
    let number_of_iterations = input.len();
    let delta_x = 3;
    let mut current_x = delta_x;
    let mut tree_count = 0;
    // Start on the second line, 4th from the left
    for i in 1..number_of_iterations {
        let line = &input[i];
        if current_x >= line.len() {
            current_x = current_x % line.len();
        }

        let position = line[current_x];
        if position == 1 {
            tree_count += 1;
        }

        current_x += delta_x;
    }

    tree_count
}

fn traverse_slope(input: &Vec<Vec<i32>>, rule: Ruleset) -> i32 {
    let mut current_x = rule.x_delta;
    let mut current_y = rule.y_delta;
    let mut tree_count = 0;

    while current_y < input.len() {
        let line = &input[current_y];
        if current_x >= line.len() {
            current_x = current_x % line.len();
        }

        let position = line[current_x];
        if position == 1 {
            tree_count += 1;
        }

        current_x += rule.x_delta;
        current_y += rule.y_delta;
    }

    tree_count
}

fn main() {
    println!("Parsing input...");
    let input = parse_input();

    println!("Calculating part one...");
    let part_one_rule = Ruleset {
        x_delta: 3,
        y_delta: 1,
    };
    let part_one_result = part_one(&input);
    let part_one_traverse = traverse_slope(&input, part_one_rule);
    println!("You hit {} trees using part_one", part_one_result);
    println!("You hit {} trees using traverse_slope", part_one_traverse);

    println!("Calculating part two...");
    let part_two_rules = vec![
        Ruleset {
            x_delta: 1,
            y_delta: 1,
        },
        Ruleset {
            x_delta: 3,
            y_delta: 1,
        },
        Ruleset {
            x_delta: 5,
            y_delta: 1,
        },
        Ruleset {
            x_delta: 7,
            y_delta: 1,
        },
        Ruleset {
            x_delta: 1,
            y_delta: 2,
        },
    ];

    let mut part_two_result: i64 = 1;
    for rule in part_two_rules {
        let trees_hit = traverse_slope(&input, rule) as i64;
        println!("You hit {} trees on this slope", trees_hit);
        part_two_result = part_two_result * trees_hit;
    }
    println!("Part two result: {}", part_two_result);
}
