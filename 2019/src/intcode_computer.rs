pub fn run_intcode(code: &mut Vec<i32>, input: i32) -> Option<i32> {
	let mut output = None;
	let mut i = 0;
	while code[i] != 99 {
		let opcode = code[i] % 100;

		let param_mode1 = (code[i] / 100) % 10;
		let param_mode2 = (code[i] / 1000) % 10;
		let param_mode3 = (code[i] / 10000) % 10;

		let arg_index1 = get_argument_index(code, i+1, param_mode1);
		let arg_index2 = get_argument_index(code, i+2, param_mode2);
		let arg_index3 = get_argument_index(code, i+3, param_mode3);

		match (opcode, arg_index1, arg_index2, arg_index3) {
			(1, Some(arg1), Some(arg2), Some(arg3)) => {
				code[arg3] = code[arg1] + code[arg2];
				i += 4;
			},
			(2, Some(arg1), Some(arg2), Some(arg3)) => {
				code[arg3] = code[arg1] * code[arg2];
				i += 4;
			},
			(3, Some(arg1), _, _) => {
				code[arg1] = input;
				i += 2;
			},
			(4, Some(arg1), _, _) => {
				output = Some(code[arg1]);
				if let Some(o) = output {
					println!("{}", o);
				}
				i += 2;
			},
			(5, Some(arg1), Some(arg2), _) => {
				if code[arg1] != 0 {
					i = code[arg2] as usize;
				} else {
					i += 3;
				}
			},
			(6, Some(arg1), Some(arg2), _) => {
				if code[arg1] == 0 {
					i = code[arg2] as usize;
				} else {
					i += 3;
				}
			},
			(7, Some(arg1), Some(arg2), Some(arg3)) => {
				code[arg3] = (code[arg1] < code[arg2]) as i32;
				i += 4;
			},
			(8, Some(arg1), Some(arg2), Some(arg3)) => {
				code[arg3] = (code[arg1] == code[arg2]) as i32;
				i += 4;
			},
			(_, Some(_), Some(_), Some(_)) => panic!("Invalid opcode at position {} for instruction: {:?}", i, code[i]),
			_ => panic!("Probably (lol) not enough arguments for opcode {:?} at position {}: {:?}{:?}{:?}", code[i], i, arg_index1, arg_index2, arg_index3),
		}
	}
	output
}

fn get_argument_index(code: &Vec<i32>, index: usize, param_mode: i32) -> Option<usize> {
	if index >= code.len() {
		return None;
	}
	match param_mode {
		0 => get_argument_index(code, code[index] as usize, 1),
		1 => Some(index),
		err => panic!("Invalid parameter mode ({}) in: {}", err, code[index]),
	}
}
