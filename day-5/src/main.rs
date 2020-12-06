use std::fs;

struct BoardingPass {
    raw_row: String,
    raw_seat: String,
}

impl BoardingPass {
    fn row(&self) -> i32 {
        let translation: String = self
            .raw_row
            .chars()
            .map(|letter| match letter {
                'F' => "0",
                'B' => "1",
                _ => "",
            })
            .collect();
        i32::from_str_radix(&translation, 2).unwrap()
    }
    fn seat(&self) -> i32 {
        let translation: String = self
            .raw_seat
            .chars()
            .map(|letter| match letter {
                'L' => "0",
                'R' => "1",
                _ => "",
            })
            .collect();
        i32::from_str_radix(&translation, 2).unwrap()
    }
    // Technically you could just convert the entire raw boarding pass e.g. "BFFFBBFRRR"
    // To a binary number e.g. 1000110111 and that is the seat_id but I don't
    // Want to go back and do it that way
    fn seat_id(&self) -> i32 {
        self.row() * 8 + self.seat()
    }
}

fn parse_input() -> Vec<BoardingPass> {
    let contents = fs::read_to_string("input").expect("Unable to read input file");
    let raw_boarding_passes = contents.lines();
    let mut boarding_passes: Vec<BoardingPass> = Vec::new();
    for raw_boarding_pass in raw_boarding_passes {
        let boarding_pass = BoardingPass {
            raw_row: raw_boarding_pass[..7].to_string(),
            raw_seat: raw_boarding_pass[7..].to_string(),
        };

        boarding_passes.push(boarding_pass);
    }

    boarding_passes
}

fn part_one(boarding_passes: &Vec<BoardingPass>) -> i32 {
    let mut max = 0;
    for boarding_pass in boarding_passes {
        let seat_id = boarding_pass.seat_id();
        if seat_id > max {
            max = seat_id;
        }
    }

    max
}

fn part_two(mut boarding_passes: Vec<BoardingPass>) -> i32 {
    boarding_passes.sort_by(|a, b| a.seat_id().cmp(&b.seat_id()));

    for (i, boarding_pass) in boarding_passes.iter().enumerate() {
        if i == 0 || i == boarding_passes.len() {
            continue;
        }

        let seat_id = boarding_pass.seat_id();
        let prev_seat_id = boarding_passes[i - 1].seat_id();
        if (seat_id - 1) != prev_seat_id {
            return seat_id - 1;
        }

        let next_seat_id = boarding_passes[i + 1].seat_id();
        if (seat_id + 1) != next_seat_id {
            return seat_id + 1;
        }
    }

    -1
}

fn main() {
    let boarding_passes = parse_input();

    let highest_seat_id = part_one(&boarding_passes);
    println!("The highest seat ID is {}", highest_seat_id);

    let my_seat_id = part_two(boarding_passes);
    println!("My seat ID is {}", my_seat_id);
}
