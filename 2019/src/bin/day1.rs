// Advent of Code 2019 - day 1
// https://adventofcode.com/2019/day/1

// Takes a text file with one integer on each line as input

fn main() {
	let file = std::env::args().nth(1).expect("");
	let path = std::path::Path::new(&file);
	let content = std::fs::read_to_string(path).unwrap();

	let mut module_fuel_required = 0;
	let mut total_fuel_required = 0;
	for line in content.lines() {
		let module_mass = line.parse::<i32>().unwrap();
		module_fuel_required += fuel_req(module_mass);
		total_fuel_required += fuel_req_recursive(module_mass);
	}
	println!("Part 1 - The sum of the fuel requirements for all of the modules on your spacecraft is: {}", module_fuel_required);
	println!("Part 2 - Accounting for the fuel mass as well, the sum is: {}", total_fuel_required);
}

// Calculate fuel requirement for some mass
fn fuel_req(mass: i32) -> i32 {
	mass / 3 - 2
}

// Calculate fuel requirement for some mass, accounting for the weight of the fuel itself
fn fuel_req_recursive(mass: i32) -> i32 {
	let fuel = fuel_req(mass);
	if fuel <= 0 {
		0
	} else {
		fuel + fuel_req_recursive(fuel)
	}
}
