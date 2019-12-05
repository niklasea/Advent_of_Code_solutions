// Advent of Code 2019 - day 5
// https://adventofcode.com/2019/day/5

// Takes as input a text file with a comma-separated intcode

mod intcode_computer;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	// TODO: Use '-f' for file input or '-' for standard input?
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
	if let Some(res) = intcode_computer::run_intcode(program, input) {
		res
	} else {
		program[0]
	}
}
