// Advent of Code 2019 - day 3
// https://adventofcode.com/2019/day/3

// Takes as input either
// 1. a text file with two comma-separated wire paths on separate lines 
// 2. two comma-separated wire paths

#[derive(Copy, Clone, Debug)]
struct Point {
	x: i32,
	y: i32,
	steps: i32,
}

impl std::fmt::Display for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "(x: {}, y: {}, steps: {})", self.x, self.y, self.steps)
	}
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let file_string = match args.len() {
		3 => {
			args[1].to_string() + "\n" + &args[2]
		},
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

	println!("{}", file_string);
	let mut wires = Vec::new();
	for line in file_string.lines() {
		// Trim whitespace, split on commas, parse and put the result in a vector
		let line_vectors: Vec<Point> = 
			line
			.trim()
			.split(',')
			.map(|s| parse_instruction(s.to_string()))
			.collect();
		wires.push(build_path(line_vectors));
	}

	let mut intersections = Vec::new();
	let mut last_p: Point = Point {x: 0, y: 0, steps: 0};
	for p in wires[0].to_vec() {
		let mut last_q: Point = Point {x: 0, y: 0, steps: 0};
		for q in wires[1].to_vec() {
			let point = find_intersection(last_p, p, last_q, q);
			if let Some(res) = point {
				intersections.push(res)
			}
			last_q = q;
		}
		last_p = p;
	}

	let mut closest = intersections[0];
	let mut fewest = intersections[0];
	for i in intersections {
		if (i.x.abs() + i.y.abs()) < (closest.x.abs() + closest.y.abs()) {
			closest = i;
		}
		if i.steps < fewest.steps {
			fewest = i;
		}
	}
	println!("Part 1 - The intersection closest to the central port is at {}, at a distance of {}", closest, closest.x.abs() + closest.y.abs());
	println!("Part 2 - The intersection fewest steps from the central port is at {}", fewest);
}

fn parse_instruction(instruction: String) -> Point {
	let (direction, length_str) = instruction.split_at(1);
	let length = match length_str.parse::<i32>() {
		Err(why) => panic!("Unable to parse number \"{}\", {}", length_str, std::error::Error::description(&why)),
		Ok(n) => n,
	};
	match direction {
		"R" => Point { x: length, y: 0, steps: length},
		"D" => Point { x: 0, y: -length, steps: length },
		"L" => Point { x: -length, y: 0, steps: length },
		"U" => Point { x: 0, y: length, steps: length },
		d => panic!("Input file is not formatted correctly, {} is not a valid direction (instruction {})", d, instruction),
	}
}

fn build_path(vectors: Vec<Point>) -> Vec<Point> {
	let mut last: Point = Point {x: 0, y: 0, steps: 0};
	let mut path: Vec<Point> = Vec::new();
	for v in vectors {
		last = Point {x: last.x + v.x, y: last.y + v.y, steps: last.steps + v.steps};
		path.push(last);
	}
	path
}

// Uses the algorithm from Wikipedia
// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
fn find_intersection(p_start: Point, p_end: Point, q_start: Point, q_end: Point) -> Option<Point> {
	let denominator = (p_start.x - p_end.x) * (q_start.y - q_end.y) - (p_start.y - p_end.y) * (q_start.x - q_end.x);
	if denominator == 0 {
		return None;
	}
	// Return if this is the central port or the denominator is 0 (parallel lines)
	match (p_start.x, p_start.y, q_start.x, q_start.y, denominator) {
		(_, _, _, _, 0) => return None,
		(0, 0, 0, 0, _) => return None,
		_ => (),
	}
	let t = ((p_start.x - q_start.x) * (q_start.y - q_end.y) - (p_start.y - q_start.y) * (q_start.x - q_end.x)) as f64 / denominator as f64;
	let u = - ((p_start.x - p_end.x) * (p_start.y - q_start.y) - (p_start.y - p_end.y) * (p_start.x - q_start.x)) as f64 / denominator as f64;
	if 0f64 <= t && t <= 1f64 && 0f64 <= u && u <= 1f64 {
		let intersection_x = p_start.x + (t * (p_end.x - p_start.x) as f64).round() as i32;
		let intersection_y = p_start.y + (t * (p_end.y - p_start.y) as f64).round() as i32;
		let p_steps = p_start.steps + (p_start.x - intersection_x).abs() + (p_start.y - intersection_y).abs();
		let q_steps = q_start.steps + (q_start.x - intersection_x).abs() + (q_start.y - intersection_y).abs();
		Some(Point {
			x: intersection_x,
			y: intersection_y,
			steps: p_steps + q_steps,
		})
	} else {
		None
	}
}
