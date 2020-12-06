// Advent of Code 2019 - day 7
// https://adventofcode.com/2019/day/7

// Takes as input a text file with a comma-separated intcode

use aoc2019::intcode_computer;

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

	let mut part_one = 0;
	let mut part_one_phases: Option<Vec<i32>> = None;
	let iter = (0..=4)
		.flat_map(|e| std::iter::repeat(e).zip(0..=4))
		.flat_map(|e| std::iter::repeat(e).zip(0..=4))
		.flat_map(|e| std::iter::repeat(e).zip(0..=4))
		.flat_map(|e| std::iter::repeat(e).zip(0..=4));
	for ((((a, b), c), d), e) in iter {
		// Skip if a phase setting is used more than once
		let phase_settings: std::collections::HashSet<i32> = [ a, b, c, d, e ].iter().cloned().collect();
		if phase_settings.len() < 5 {
			continue;
		}

		let res = intcode_computer::run_intcode(&mut content_array.to_vec(), &mut vec![a, 0], 0);
		let res = 
			if let (_, Some(r)) = res {
				intcode_computer::run_intcode(&mut content_array.to_vec(), &mut vec![b, r], 0)
			} else {
				(0, None)
			};
		let res =
			if let (_, Some(r)) = res {
				intcode_computer::run_intcode(&mut content_array.to_vec(), &mut vec![c, r], 0)
			} else {
				(0, None)
			};
		let res =
			if let (_, Some(r)) = res {
				intcode_computer::run_intcode(&mut content_array.to_vec(), &mut vec![d, r], 0)
			} else {
				(0, None)
			};
		let res =
			if let (_, Some(r)) = res {
				intcode_computer::run_intcode(&mut content_array.to_vec(), &mut vec![e, r], 0)
			} else {
				(0, None)
			};
		if let (_, Some(r)) = res {
			if r > part_one {
				part_one = r;
				part_one_phases = Some(vec![a,b,c,d,e]);
			}
		}
	}
	if let Some(phases) = part_one_phases {
		println!("Part 1 - Largest thruster signal of {} found with phase setting {:?}", part_one, phases);
	}

	// let amp_a = &mut content_array.to_vec();
	// let amp_b = &mut content_array.to_vec();
	// let amp_c = &mut content_array.to_vec();
	// let amp_d = &mut content_array.to_vec();
	// let amp_e = &mut content_array.to_vec();
	// let mut part_two = 0;
	// let mut part_two_phases: Option<Vec<i32>> = None;
	// let iter = (5..=9)
	// 	.flat_map(|e| std::iter::repeat(e).zip(5..=9))
	// 	.flat_map(|e| std::iter::repeat(e).zip(5..=9))
	// 	.flat_map(|e| std::iter::repeat(e).zip(5..=9))
	// 	.flat_map(|e| std::iter::repeat(e).zip(5..=9));
}
