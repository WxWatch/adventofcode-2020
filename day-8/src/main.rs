use std::fs;

#[derive(Clone)]
struct Instruction {
    verb: String,
    amount: i32,
}

fn parse_input() -> Vec<Instruction> {
    let contents = fs::read_to_string("input").expect("Unable to read input file");
    let raw_instructions = contents.lines();
    let mut instructions = Vec::new();

    for raw_instruction in raw_instructions {
        let raw_groups: Vec<String> = raw_instruction.split(" ").map(str::to_string).collect();
        let instruction = Instruction {
            verb: raw_groups[0].to_string(),
            amount: raw_groups[1].parse().unwrap(),
        };

        instructions.push(instruction);
    }

    instructions
}

fn part_one(instructions: &Vec<Instruction>) -> i32 {
    let mut acc = 0;
    let mut idx = 0i32;
    let mut executed_instructions = vec![];

    loop {
        if executed_instructions.contains(&idx) {
            break;
        }

        let instruction_to_execute = &instructions[idx as usize];
        executed_instructions.push(idx);
        match instruction_to_execute.verb.as_str() {
            "acc" => {
                acc += instruction_to_execute.amount;
                idx += 1
            }
            "jmp" => idx += instruction_to_execute.amount,
            _ => idx += 1,
        }
    }
    acc
}

fn run_instructions(instructions: &Vec<Instruction>) -> i32 {
    let mut acc = 0;
    let mut idx = 0i32;
    let mut executed_instructions = vec![];

    loop {
        if executed_instructions.contains(&idx) {
            println!("Detected infinite loop");
            return -1;
        }

        if idx == instructions.len() as i32 {
            println!("Exiting normally");
            break;
        }

        let instruction_to_execute = &instructions[idx as usize];
        executed_instructions.push(idx);
        match instruction_to_execute.verb.as_str() {
            "acc" => {
                acc += instruction_to_execute.amount;
                idx += 1
            }
            "jmp" => idx += instruction_to_execute.amount,
            _ => idx += 1,
        }
    }
    acc
}

fn part_two(instructions: &Vec<Instruction>) -> i32 {
    let mut idx = 0usize;
    loop {
        if idx == instructions.len() {
            println!("Exiting normally");
            break;
        }
        // Copy instructions so we can mutate
        let mut copied_instructions = instructions.to_vec();
        let mut current_instruction = copied_instructions.get_mut(idx).unwrap();
        let flipped = flip_instruction(&mut current_instruction);
        if flipped {
            let result = run_instructions(&copied_instructions);
            if result > -1 {
                return result;
            }
        }

        idx += 1;
    }
    -1
}

fn flip_instruction(instruction: &mut Instruction) -> bool {
    match instruction.verb.as_str() {
        "nop" => {
            instruction.verb = "jmp".to_string();
            true
        }
        "jmp" => {
            instruction.verb = "nop".to_string();
            true
        }
        _ => false,
    }
}

fn main() {
    let instructions = parse_input();
    let accumulator = part_one(&instructions);
    println!("Acc is {}", accumulator);

    let solution = part_two(&instructions);
    println!("Acc is {}", solution);
}
