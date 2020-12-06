// Advent of Code 2019 - day 3
// https://adventofcode.com/2019/day/3

// Takes as input either
// 1. a text file with two comma-separated wire paths on separate lines
// 2. two comma-separated wire paths as arguments

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

const CENTRAL_PORT: Point = Point{ x: 0, y: 0, steps: 0 };

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

	let mut closest: Option<Point> = None;
	let mut fewest: Option<Point> = None;
	let mut last_p: Point = CENTRAL_PORT;
	let mut last_q: Point = CENTRAL_PORT;
	for p in wires[0].to_vec() {
		for q in wires[1].to_vec() {
			let point = find_intersection(last_p, p, last_q, q);
			match (point, closest, fewest) {
				(Some(p), Some(c), Some(f)) => {
					if manhattan_distance(CENTRAL_PORT, p) < manhattan_distance(CENTRAL_PORT, c) {
						closest = point;
					}
					if p.steps < f.steps {
						fewest = point;
					}
				},
				(Some(_), None, None) => {
					closest = point;
					fewest = point;
				},
				_ => (),
			}
			last_q = q;
		}
		last_p = p;
	}

	if let Some(result_one) = closest {
		println!("Part 1 - The intersection closest to the central port is at {}, at a distance of {}", result_one, manhattan_distance(CENTRAL_PORT, result_one));
	}
	if let Some(result_two) = fewest {
		println!("Part 2 - The intersection fewest steps from the central port is at {}", result_two);
	}

}

fn parse_instruction(instruction: String) -> Point {
	let (direction, length_str) = instruction.split_at(1);
	let length = match length_str.parse::<i32>() {
		Err(why) => panic!("Unable to parse number \"{}\", {}", length_str, std::error::Error::description(&why)),
		Ok(n) => n,
	};
	match direction {
		"R" => Point{ x: length, y: 0, steps: length },
		"D" => Point{ x: 0, y: -length, steps: length },
		"L" => Point{ x: -length, y: 0, steps: length },
		"U" => Point{ x: 0, y: length, steps: length },
		d => panic!("Input file is not formatted correctly, {} is not a valid direction (instruction {})", d, instruction),
	}
}

fn build_path(vectors: Vec<Point>) -> Vec<Point> {
	let mut last: Point = Point { x: 0, y: 0, steps: 0 };
	let mut path: Vec<Point> = Vec::new();
	for v in vectors {
		last = Point{ x: last.x + v.x, y: last.y + v.y, steps: last.steps + v.steps };
		path.push(last);
	}
	path
}

fn manhattan_distance(start: Point, end: Point) -> i32 {
	(end.y - start.y).abs() + (end.x - start.x).abs()
}

// Uses the algorithm from Wikipedia
// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line
// TODO: Make this method more generic
fn find_intersection(p_start: Point, p_end: Point, q_start: Point, q_end: Point) -> Option<Point> {
	let Point{ x: x1, y: y1, .. } = p_start;
	let Point{ x: x2, y: y2, .. } = p_end;
	let Point{ x: x3, y: y3, .. } = q_start;
	let Point{ x: x4, y: y4, .. } = q_end;

	let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
	// Skip if this is the central port or the denominator is 0 (parallel lines)
	match (x1, y1, x3, y3, denominator) {
		(_, _, _, _, 0) => return None,
		(0, 0, 0, 0, _) => return None,
		_ => (),
	}
	let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) as f64 / denominator as f64;
	let u = - ((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) as f64 / denominator as f64;

	if 0.0 <= t && t <= 1.0 && 0.0 <= u && u <= 1.0 {
		let mut intersection = Point{
			x: x1 + (t * (x2 - x1) as f64).round() as i32,
			y: y1 + (t * (y2 - y1) as f64).round() as i32,
			steps: 0
		};
		let p_steps = p_start.steps + manhattan_distance(p_start, intersection);
		let q_steps = q_start.steps + manhattan_distance(q_start, intersection);
		intersection.steps = p_steps + q_steps;
		Some(intersection)
	} else {
		None
	}
}
