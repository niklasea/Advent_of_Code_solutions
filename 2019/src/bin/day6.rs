// Advent of Code 2019 - day 6
// https://adventofcode.com/2019/day/6

// Takes a text file with one "orbital relationship" on each line as input

struct OrbitalObject {
	parent: Option<String>,
	children: Vec<String>,
}

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
	orbits.insert("COM".to_string(), OrbitalObject{ parent: None, children: Vec::new() });
	for line in input_string.lines() {
		let mut orbit_strings = line.split(")");
		let (center, satelite) = match (orbit_strings.next(), orbit_strings.next()) {
			(Some(c), Some(s)) => (c, s),
			_ => panic!("ERROR: Wrong orbit format"),
		};
		match orbits.get(satelite) {
			Some(s) => {
				let new_object = OrbitalObject{ parent: Some(center.to_string()), children: s.children.to_vec() };
				orbits.insert(satelite.to_string(), new_object);
			},
			None => {
				orbits.insert(satelite.to_string(), OrbitalObject{ parent: Some(center.to_string()), children: Vec::new() });
			},
		}
		match orbits.get_mut(center) {
			Some(c) => c.children.push(satelite.to_string()),
			None => {
				orbits.insert(center.to_string(), OrbitalObject{ parent: None, children: vec![satelite.to_string()] });
			},
		}
	}

	println!("Part 1 - The total number of direct and indirect orbits is {}", count_orbits(&orbits, "COM", 0));

	let src = orbits.get("YOU").unwrap().parent.as_ref().unwrap();
	let dest = orbits.get("SAN").unwrap().parent.as_ref().unwrap();
	if let Some(res) = count_transfers(&orbits, src, dest) {
		println!("Part 2 - The minimum number of orbital transfers for 'YOU' to orbit the same object as 'SAN' is {}", res);
	}
}

fn count_orbits(orbits: &std::collections::HashMap<String, OrbitalObject>, key: &str, depth: i32) -> i32 {
	orbits.get(key)
		.unwrap()
		.children
		.iter()
		.fold(depth, |sum, v| sum + count_orbits(orbits, v, depth + 1))
}

fn count_transfers(orbits: &std::collections::HashMap<String, OrbitalObject>, start: &str, dest: &str) -> Option<i32> {
	let start_path = find_path_to_com(orbits, start);
	let dest_path = find_path_to_com(orbits, dest);
	let mut transfers = None;
	'outer: for s in 0..start_path.len() {
		for d in 0..dest_path.len() {
			if start_path[s] == dest_path[d] {
				transfers = Some((s + d) as i32);
				break 'outer;
			}
		}
	}
	transfers
}

fn find_path_to_com(orbits: &std::collections::HashMap<String, OrbitalObject>, start: &str) -> Vec<String> {
	let mut path = vec![start.to_string()];
	let mut cur = &orbits.get(start).unwrap().parent;
	while let Some(p) = cur {
		path.push(p.to_string());
		cur = &orbits.get(p).unwrap().parent;
	}
	path
}
