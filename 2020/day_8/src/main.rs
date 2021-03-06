use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Clone, Copy)]
struct Instruction {
    operation: Operation,
    argument: i64,
}

fn main() {
    let input = std::fs::read("./input.txt").expect("Unable to read input file.");
    let input = String::from_utf8(input).expect("Input file is not valid UTF-8.");

    let instructions: Vec<Instruction> = input.lines()
        .map(|x| {
            let input: Vec<&str> = x.split_whitespace().collect();
            match (input[0], input[1].parse::<i64>().expect("Invalid instruction argument.")) {
                ("nop", _) => Instruction { operation: Operation::Nop, argument: 0 },
                ("acc", arg) => Instruction { operation: Operation::Acc, argument: arg },
                ("jmp", arg) => Instruction { operation: Operation::Jmp, argument: arg },
                _ => panic!("Invalid instruction."),
            }
        })
        .collect();
    
    match run_program(&instructions) {
        Ok(_) => eprintln!("The original program should loop infinitely."),
        Err(acc) => println!("The accumulator value is {} right before the loop.", acc),
    }

    for i in 0..instructions.len() {
        let mut modified_instructions = instructions.clone();
        modified_instructions[i].operation = match instructions[i].operation {
            Operation::Nop => Operation::Jmp,
            Operation::Jmp => Operation::Nop,
            _ => continue,
        };
        match run_program(&modified_instructions) {
            Ok(res) => {
                println!("After fixing the program, the result is {}.", res);
                break;
            },
            Err(_) => (),
        }
    }
}

fn run_program(instructions: &Vec<Instruction>) -> Result<i64, i64> {
    let mut visited = HashSet::new();
    let mut accumulator = 0;
    let mut pointer = 0;
    while visited.insert(pointer) {
        if pointer == instructions.len() {
            return Ok(accumulator);
        }
        match (instructions[pointer as usize].operation, instructions[pointer as usize].argument) {
            (Operation::Acc, arg) => {
                accumulator += arg;
                pointer += 1
            },
            (Operation::Jmp, arg) => pointer = (pointer as i64 + arg) as usize,
            (Operation::Nop, _) => pointer += 1,
        }
    }
    Err(accumulator)
}
