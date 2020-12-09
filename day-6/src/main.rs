use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

struct Group {
    answers: Vec<String>,
}

impl Group {
    fn unique_answers(&self) -> usize {
        let mut uniques = HashSet::new();
        for answer in &self.answers {
            for c in answer.chars() {
                uniques.insert(c);
            }
        }
        uniques.len()
    }

    fn unanimous_answers(&self) -> i32 {
        let answer_counts: HashMap<String, i32> =
            self.answers.iter().fold(HashMap::new(), |mut map, x| {
                for c in x.chars() {
                    match map.get(&c.to_string()) {
                        // There's actually a warning here about borrowing as
                        // mutable after borrowing as immutable but not sure
                        // what to do about it :thinking:
                        Some(n) => map.insert(c.to_string(), n + 1),
                        None => map.insert(c.to_string(), 1),
                    };
                }

                map
            });

        answer_counts.values().fold(0, |acc, x| {
            if *x as usize == self.answers.len() {
                return acc + 1;
            }

            acc
        })
    }
}

fn parse_input() -> Vec<Group> {
    let contents = fs::read_to_string("input").expect("Unable to read input file");
    let raw_groups = contents.split("\n\n");
    let mut groups: Vec<Group> = Vec::new();
    for raw_group in raw_groups {
        let raw_answers = raw_group.split_whitespace().map(str::to_string).collect();
        let group = Group {
            answers: raw_answers,
        };

        groups.push(group);
    }

    groups
}

fn part_one(groups: &Vec<Group>) -> usize {
    let mut sum = 0usize;
    for group in groups {
        sum += group.unique_answers();
    }

    sum
}

fn part_two(groups: &Vec<Group>) -> i32 {
    let mut sum = 0;
    for group in groups {
        sum += group.unanimous_answers();
    }

    sum
}

fn main() {
    let groups = parse_input();
    let part_one_sum = part_one(&groups);
    println!("The sum of the unique question counts is {}", part_one_sum);

    let part_two_sum = part_two(&groups);
    println!(
        "The sum of the unanimous question counts is {}",
        part_two_sum
    );
}
