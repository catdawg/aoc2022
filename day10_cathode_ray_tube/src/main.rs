use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
		
	let now = std::time::Instant::now();

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

	let mut cycle = 0;
	let mut x = 1;

	let res = iter.fold(0, |acc, command| {

		let mut acc = acc;
		match command {
			Command::Addx(arg) => {
				cycle += 1;
				if (cycle + 20) % 40 == 0 {
					acc += cycle * x;
				}
				cycle += 1;
				if (cycle + 20) % 40 == 0 {
					acc += cycle * x;
				}
				x += arg;

			},
			Command::Noop => {
				cycle += 1;
				if (cycle + 20) % 40 == 0 {
					acc += cycle * x;
				}
			},
		}
		acc
	});

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
