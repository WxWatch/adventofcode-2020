use hex;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

// This is jank just making everything a string but idc
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn has_required_fields(&self) -> bool {
        match self {
            Passport { byr: None, .. } => false,
            Passport { iyr: None, .. } => false,
            Passport { eyr: None, .. } => false,
            Passport { hgt: None, .. } => false,
            Passport { hcl: None, .. } => false,
            Passport { ecl: None, .. } => false,
            Passport { pid: None, .. } => false,
            // Passport { cid: None, .. } => false,
            _ => true,
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            Passport { byr: None, .. } => false,
            Passport { iyr: None, .. } => false,
            Passport { eyr: None, .. } => false,
            Passport { hgt: None, .. } => false,
            Passport { hcl: None, .. } => false,
            Passport { ecl: None, .. } => false,
            Passport { pid: None, .. } => false,
            // Passport { cid: None, .. } => false,
            _ => self.validate_fields(),
        }
    }

    fn validate_fields(&self) -> bool {
        let min_birth = 1920;
        let max_birth = 2002;
        let valid_birthyear =
            Passport::validate_year(self.byr.as_ref().unwrap(), min_birth, max_birth);
        if !valid_birthyear {
            return false;
        }

        let min_issue = 2010;
        let max_issue = 2020;
        let valid_issueyear =
            Passport::validate_year(self.iyr.as_ref().unwrap(), min_issue, max_issue);
        if !valid_issueyear {
            return false;
        }

        let min_exp = 2020;
        let max_exp = 2030;
        let valid_expiration =
            Passport::validate_year(self.eyr.as_ref().unwrap(), min_exp, max_exp);
        if !valid_expiration {
            return false;
        }

        let valid_height = Passport::validate_height(self.hgt.as_ref().unwrap());
        if !valid_height {
            return false;
        }

        let valid_hair = Passport::validate_hex(self.hcl.as_ref().unwrap());
        if !valid_hair {
            return false;
        }

        let valid_eyecolor = Passport::validate_eyecolor(self.ecl.as_ref().unwrap());
        if !valid_eyecolor {
            return false;
        }

        let valid_passport_id = Passport::validate_passport_id(self.pid.as_ref().unwrap());
        println!(
            "Valid pid: {}, pid: {}",
            valid_passport_id,
            self.pid.as_ref().unwrap()
        );
        if !valid_passport_id {
            return false;
        }

        true
    }

    fn validate_year(yr: &String, min: i32, max: i32) -> bool {
        if let Ok(n) = yr.parse::<i32>() {
            if n >= min && n <= max {
                return true;
            }
        }
        false
    }

    fn validate_hex(hex: &String) -> bool {
        if hex.len() != 7 {
            return false;
        }

        if &hex[0..1] != "#" {
            return false;
        }

        if let Err(_) = hex::decode(&hex[1..]) {
            return false;
        }

        true
    }

    fn validate_height(hgt: &String) -> bool {
        let cm_min = 150;
        let cm_max = 193;
        let in_min = 59;
        let in_max = 76;

        let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        if let Some(captures) = re.captures(&hgt) {
            if let Ok(value) = captures[1].parse::<i32>() {
                let unit = &captures[2];
                match unit {
                    "in" => return value >= in_min && value <= in_max,
                    "cm" => return value >= cm_min && value <= cm_max,
                    _ => false,
                };
            }
        }
        false
    }

    fn validate_eyecolor(ecl: &String) -> bool {
        // I'm lazy
        match ecl.as_str() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        }
    }

    fn validate_passport_id(pid: &String) -> bool {
        let re = Regex::new(r"^\d{9}$").unwrap();
        re.is_match(&pid)
    }
}

fn parse_input() -> Vec<Passport> {
    let contents = fs::read_to_string("input").expect("Unable to read input file");
    let raw_passports = contents.split("\n\n");
    let mut passports: Vec<Passport> = Vec::new();
    for raw_passport in raw_passports {
        let raw_fields = raw_passport.split_whitespace();
        let mut fields = HashMap::new();
        for raw_field in raw_fields {
            let tokens: Vec<String> = raw_field.split(":").map(str::to_string).collect();
            fields.insert(tokens[0].clone(), tokens[1].clone());
        }

        let passport = Passport {
            byr: fields.get("byr").cloned(),
            iyr: fields.get("iyr").cloned(),
            eyr: fields.get("eyr").cloned(),
            hgt: fields.get("hgt").cloned(),
            hcl: fields.get("hcl").cloned(),
            ecl: fields.get("ecl").cloned(),
            pid: fields.get("pid").cloned(),
            cid: fields.get("cid").cloned(),
        };

        passports.push(passport);
    }

    passports
}

fn part_one(passports: &Vec<Passport>) -> i32 {
    let mut valid_passports = 0;
    for passport in passports {
        if passport.has_required_fields() {
            valid_passports += 1;
        }
    }
    valid_passports
}

fn part_two(passports: &Vec<Passport>) -> i32 {
    let mut valid_passports = 0;
    for passport in passports {
        if passport.is_valid() {
            valid_passports += 1;
        }
    }
    valid_passports
}

fn main() {
    println!("Hello, world!");
    let passports = parse_input();
    let p1_valid_passports = part_one(&passports);
    println!("Part 1: There are {} valid passports", p1_valid_passports);

    let p2_valid_passports = part_two(&passports);
    println!("Part 2: There are {} valid passports", p2_valid_passports);
}
