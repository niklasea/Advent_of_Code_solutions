// Advent of Code 2019 - day 6
// https://adventofcode.com/2019/day/6

// Takes a text file with one "orbital relationship" on each line as input

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let input_string = match args.len() {
		2 => {
			let path = std::path::Path::new(&args[1]);
			match std::fs::read_to_string(path) {
				Err(why) => panic!("Could not read {}: {}", path.display(), std::error::Error::description(&why)),
				Ok(string) => string,
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

	let mut orbits = std::collections::HashMap::new();
	orbits.insert("COM", Vec::new());
	for line in input_string.lines() {
		let mut orbit_strings = line.split(")");
		let (center, satelite) = match (orbit_strings.next(), orbit_strings.next()) {
			(Some(c), Some(s)) => (c, s),
			_ => panic!("ERROR: Wrong orbit format"),
		};
		if let None = orbits.get(satelite) {
			orbits.insert(satelite, Vec::new());
		}
		match orbits.get_mut(center) {
			Some(c) => c.push(satelite),
			None => {
				orbits.insert(center, vec![satelite]);
			},
		}
	}

	println!("Part 1 - The total number of direct and indirect orbits is {}", count_orbits(&orbits, "COM", 0));
}

fn count_orbits(orbits: &std::collections::HashMap<&str, Vec<&str>>, key: &str, depth: i32) -> i32 {
	orbits.get(key)
		.unwrap()
		.iter()
		.fold(depth, |sum, v| sum + count_orbits(orbits, v, depth + 1))
}
