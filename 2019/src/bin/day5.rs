// Advent of Code 2019 - day 5
// https://adventofcode.com/2019/day/5

// Takes as input a text file with a comma-separated intcode

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let intcode_string = match args.len() {
		2 => {
			let path = std::path::Path::new(&args[1]);
			match std::fs::read_to_string(path) {
				Err(why) => panic!("Could not read {}: {}", path.display(), std::error::Error::description(&why)),
				Ok(string) => {
					let mut str_iter = string.lines();
					str_iter.next().unwrap().to_string()
				}
			}
		},
		1 => {
			eprintln!("ERROR: No input file specified");
			return;
		},
		_ => {
			eprintln!("ERROR: Too many arguments");
			return;
		},
	};
	
	// Trim whitespace, split on commas, parse as integers and put the result in a vector
	let content_array: Vec<i32> = intcode_string.split(',').map(|s| s.parse::<i32>().unwrap()).collect();

	let part_one = execute_program(&content_array, 1);
	println!("Part 1 - The diagnostic code for the ship's air conditioner unit is {}", part_one);
	let part_two = execute_program(&content_array, 5);
	println!("Part 2 - The diagnostic code for the ship's thermal radiator controller is {}", part_two);
}

fn execute_program(intcode: &Vec<i32>, input: i32) -> i32 {
	let program = &mut intcode.to_vec();
	run_intcode(program, input)
}

fn run_intcode(code: &mut Vec<i32>, input: i32) -> i32 {
	let mut output = 0;
	let mut i = 0;
	while code[i] != 99 {
		let opcode = code[i] % 100;
		let parmode1 = (code[i] / 100) % 10;
		let parmode2 = (code[i] / 1000) % 10;
		let parmode3 = (code[i] / 10000) % 10;
		// println!("Instruction: {}, parameter mode ({}, {}, {})", code[i], parmode1, parmode2, parmode3);
		let argindex1 = match parmode1 {
			0 => code[i+1] as usize,
			1 => i+1,
			err => panic!("Invalid parameter mode ({}) in: {}", err, code[i]),
		};
		let argindex2 = match parmode2 {
			0 => code[i+2] as usize,
			1 => i+2,
			err => panic!("Invalid parameter mode ({}) in: {}", err, code[i]),
		};
		let argindex3 = match parmode3 {
			0 => code[i+3] as usize,
			err => panic!("Invalid parameter mode ({}) in: {}", err, code[i]),
		};
		match opcode {
			1 => {
				code[argindex3] = code[argindex1] + code[argindex2];
				i += 4;
			},
			2 => {
				code[argindex3] = code[argindex1] * code[argindex2];
				i += 4;
			},
			3 => {
				code[argindex1] = input;
				i += 2;
			},
			4 => {
				output = code[argindex1];
				println!("{}", output);
				i += 2;
			},
			5 => {
				if code[argindex1] != 0 {
					i = code[argindex2] as usize;
				} else {
					i += 3;
				}
			},
			6 => {
				if code[argindex1] == 0 {
					i = code[argindex2] as usize;
				} else {
					i += 3;
				}
			},
			7 => {
				code[argindex3] = (code[argindex1] < code[argindex2]) as i32;
				i += 4;
			},
			8 => {
				code[argindex3] = (code[argindex1] == code[argindex2]) as i32;
				i += 4;
			},
			_ => panic!("Invalid opcode at position {}: {}", i, code[i]),
		}
	}
	output
}
