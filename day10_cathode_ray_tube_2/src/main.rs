use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");

	#[derive(Copy, Clone)]
	enum Command {
		Addx(i32),
		Noop,
	}

	let iter = input.lines().map(|line| {
		let mut split = line.split(" ");
		match split.next().unwrap() {
			"addx" => Command::Addx(split.next().unwrap().parse().unwrap()),
			"noop" => Command::Noop,
			_ => panic!()
		}
	});

	let mut cycle = -1;
	let mut x = 1;

	fn draw_pixel (cycle: i32, x: i32) {
		let col = cycle % 40;

		let pixel_diff = x - col;

		if col == 0 {
			println!();
		}

		if pixel_diff >= -1 && pixel_diff <= 1 {
			print!("#");
		} else {
			print!(".");
		}
	}

 	for command in iter {
		match command {
			Command::Addx(arg) => {
				cycle += 1;
				
				draw_pixel(cycle, x);

				cycle += 1;

				draw_pixel(cycle, x);

				x += arg;

			},
			Command::Noop => {
				cycle += 1;
				draw_pixel(cycle, x);
			},
		}
	};
}
