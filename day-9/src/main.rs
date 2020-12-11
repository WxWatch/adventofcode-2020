use std::fs;

fn parse_input() -> Vec<usize> {
    let contents = fs::read_to_string("input").expect("Unable to read input file");
    contents.lines().map(|x| x.parse().unwrap()).collect()
}

fn part_one(input: &Vec<usize>, preamble: usize) -> usize {
    let mut idx = preamble;
    'main: loop {
        if idx == input.len() {
            break;
        }

        let number_to_check = input[idx];
        let possible_numbers = input[idx - preamble..idx].to_vec();
        for outer_number in &possible_numbers {
            for inner_number in &possible_numbers {
                if (outer_number + inner_number) == number_to_check {
                    idx += 1;
                    continue 'main;
                }
            }
        }

        // If we get here, something bad happened
        println!("This number doesn't work: {}", number_to_check);
        return number_to_check;
    }

    0
}

fn part_two(input: &Vec<usize>, offender: usize) -> usize {
    for (i, number) in input.iter().enumerate() {
        let remaining_numbers = input[i..].to_vec();
        let mut j = 1;
        let mut sum = *number;
        while j < remaining_numbers.len() {
            let next_number = remaining_numbers[j];
            sum += next_number;
            if sum < offender {
                j += 1;
                continue;
            } else if sum > offender {
                break;
            } else {
                // we have the right numbers
                let mut the_right_numbers = input[i..i + j].to_vec();
                the_right_numbers.sort();
                return the_right_numbers.first().unwrap() + the_right_numbers.last().unwrap();
            }
        }
    }

    0
}

fn main() {
    let input = parse_input();
    let offender = part_one(&input, 25);
    let weakness = part_two(&input, offender);
    println!("The weakness is {}", weakness);
}
