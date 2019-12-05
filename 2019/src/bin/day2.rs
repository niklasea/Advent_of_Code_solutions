// Advent of Code 2019 - day 2
// https://adventofcode.com/2019/day/2

// Takes a text file with comma-separated integers as input

mod intcode_computer;

const OUTPUT: i32 = 19690720;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let file = match args.len() {
		2 => &args[1],
		1 => {
			eprintln!("ERROR: No input file specified");
			return;
		},
		_ => {
			eprintln!("ERROR: Too many arguments");
			return;
		},
	};
	let path = std::path::Path::new(file);
	let content_string = match std::fs::read_to_string(path) {
		Err(why) => panic!("Could not read {}: {}", path.display(), std::error::Error::description(&why)),
		Ok(string) => string,
	};

	// Trim whitespace, split on commas, parse as integers and put the result in a vector
	let content_array: Vec<i32> = content_string.trim().split(',').map(|s| s.parse::<i32>().unwrap()).collect();

	let part_one = execute_program(&content_array, 12, 2);
	println!("Part 1 - The restored 1202 gravity assist program produces a value of {} in position 0", part_one);

	let mut part_two = None;
	'outer: for noun in 0..=99 {
		for verb in 0..=99 {
			if execute_program(&content_array, noun, verb) == OUTPUT {
				part_two = Some(100 * noun + verb);
				break 'outer;
			}
		}
	}
	match part_two {
		Some(result) => println!("Part 2 - The {} program produces the desired output of {} in position 0", result, OUTPUT),
		None => println!("Part 2 - The program did not produce the desired output ({}) for any valid combination of noun and verb", OUTPUT),
	}
}

fn execute_program(intcode: &Vec<i32>, noun: i32, verb: i32) -> i32 {
	let program = &mut intcode.to_vec();
	program[1] = noun;
	program[2] = verb;
	if let Some(res) = intcode_computer::run_intcode(program, 0) {
		res
	} else {
		program[0]
	}
}
