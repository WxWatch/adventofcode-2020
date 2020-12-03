use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Password {
    min_chars: i32,
    max_chars: i32,
    character: String,
    text: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let re = Regex::new(&self.character).unwrap();
        let matches: i32 = re.find_iter(&self.text).count() as i32;
        self.min_chars <= matches && self.max_chars >= matches
    }

    fn is_new_valid(&self) -> bool {
        let chars: Vec<_> = self.text.chars().collect();
        let min = (self.min_chars - 1) as usize;
        let max = (self.max_chars - 1) as usize;

        (String::from(chars[min]) == self.character || String::from(chars[max]) == self.character)
            && String::from(chars[min]) != String::from(chars[max])
    }
}

fn parse_input() -> Vec<Password> {
    let contents = fs::read_to_string("input").expect("Unable to read input file");
    let lines = contents.lines();

    let mut passwords = Vec::new();

    for line in lines {
        let split = line.split(' ');
        let parts: Vec<&str> = split.collect();
        if parts.len() < 3 {
            println!("A password didn't have the right number of stuff")
        }

        let char_range = parts[0];
        let ranges: Vec<&str> = char_range.split("-").collect();
        let character = parts[1];
        let password = parts[2];

        passwords.push(Password {
            min_chars: ranges[0].parse().unwrap(),
            max_chars: ranges[1].parse().unwrap(),
            character: character.chars().next().unwrap().to_string(),
            text: password.to_string(),
        })
    }
    passwords
}

fn part_one(passwords: &Vec<Password>) {
    let mut valids = 0;
    let mut invalids = 0;
    for password in passwords {
        if password.is_valid() {
            valids += 1;
        } else {
            invalids += 1;
        }
    }
    println!("{} valid passwords using is_valid", valids);
    println!("{} invalid passwords using is_valid", invalids);
}

fn part_two(passwords: &Vec<Password>) {
    let mut valids = 0;
    let mut invalids = 0;
    for password in passwords {
        if password.is_new_valid() {
            valids += 1;
        } else {
            invalids += 1;
        }
    }
    println!("{} valid passwords using is_new_valid", valids);
    println!("{} invalid passwords using is_new_valid", invalids);
}

fn main() {
    let passwords = parse_input();
    part_one(&passwords);
    part_two(&passwords);
}
