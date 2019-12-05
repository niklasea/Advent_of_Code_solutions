// Advent of Code 2019 - day 4
// https://adventofcode.com/2019/day/4

// Takes as input integers representing the start and end of the allowed range

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let (start, end): (i32, i32) = match args.len() {
		3 => {
			(args[1].parse::<i32>().unwrap(), args[2].parse::<i32>().unwrap())
		},
		2 => {
			eprintln!("ERROR: Not enough arguments, needs both start and end");
			return;
		},
		1 => {
			eprintln!("ERROR: No input given");
			return;
		},
		_ => {
			eprintln!("ERROR: Too many arguments");
			return;
		},
	};

	let mut sum_of_valid_passwords_one = 0;
	let mut sum_of_valid_passwords_two = 0;
	for i in start..end {
		let mut same_adjacent = false;
		let mut group_of_two = false;
		let mut number_of_adjacent = 1;
		let mut increasing_digits = true;
		let mut last_digit = i % 10;
		let mut remaining = i;
		while remaining > 9 {
			remaining /= 10;
			let current_digit = remaining % 10;
			if current_digit > last_digit {
				increasing_digits = false;
			}
			if current_digit == last_digit {
				same_adjacent = true;
				number_of_adjacent += 1;
				if number_of_adjacent == 2 && remaining <= 10 {
					group_of_two = true;
				}
			} else {
				if number_of_adjacent == 2 {
					group_of_two = true;
				}
				number_of_adjacent = 1;
			}
			last_digit = current_digit;
		}
		if increasing_digits && same_adjacent {
			sum_of_valid_passwords_one += 1;
			if group_of_two {
				sum_of_valid_passwords_two += 1;
			}
		}
	}

	println!("Part 1 - There are {} valid passwords in the range {} - {}", sum_of_valid_passwords_one, start, end);
	println!("Part 2 - There are {} valid passwords in the range {} - {}", sum_of_valid_passwords_two, start, end);
}
