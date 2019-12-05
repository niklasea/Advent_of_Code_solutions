pub fn run_intcode(code: &mut Vec<i32>, input: i32) -> Option<i32> {
	let mut output = None;
	let mut i = 0;
	while code[i] != 99 {
		let opcode = code[i] % 100;
		let parmode1 = (code[i] / 100) % 10;
		let parmode2 = (code[i] / 1000) % 10;
		let parmode3 = (code[i] / 10000) % 10;
		// println!("Instruction: {}  Opcode: {}  Parameter modes (1st, 2nd, 3rd): {}, {}, {}", code[i], opcode, parmode1, parmode2, parmode3);

		// TODO: Possible index out-of-bounds for argument two and three
		let argindex1 = match parmode1 {
			0 => code[i+1] as usize,
			1 => i+1,
			err => panic!("Invalid parameter mode ({}) in: {}", err, code[i]),
		};
		let argindex2 = match parmode2 {
			0 => code[i+2] as usize,
			1 => i+2,
			err => panic!("Invalid parameter mode ({}) in: {}", err, code[i]),
		};
		let argindex3 = match parmode3 {
			0 => code[i+3] as usize,
			err => panic!("Invalid parameter mode ({}) in: {}", err, code[i]),
		};
		match opcode {
			1 => {
				code[argindex3] = code[argindex1] + code[argindex2];
				i += 4;
			},
			2 => {
				code[argindex3] = code[argindex1] * code[argindex2];
				i += 4;
			},
			3 => {
				code[argindex1] = input;
				i += 2;
			},
			4 => {
				output = Some(code[argindex1]);
				if let Some(o) = output {
					println!("{}", o);
				}
				i += 2;
			},
			5 => {
				if code[argindex1] != 0 {
					i = code[argindex2] as usize;
				} else {
					i += 3;
				}
			},
			6 => {
				if code[argindex1] == 0 {
					i = code[argindex2] as usize;
				} else {
					i += 3;
				}
			},
			7 => {
				code[argindex3] = (code[argindex1] < code[argindex2]) as i32;
				i += 4;
			},
			8 => {
				code[argindex3] = (code[argindex1] == code[argindex2]) as i32;
				i += 4;
			},
			_ => panic!("Invalid opcode at position {}: {}", i, code[i]),
		}
	}
	output
}