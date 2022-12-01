use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let file = File::open(file_path).expect("Should have been able to read the file");

	let mut accumulator = 0;
	let mut max_calories: [u32; 4] = [0, 0, 0, 0];
	
	enum LineType {
		Calorie(u32),
		NewLine,
	}

	let now = std::time::Instant::now();

	BufReader::new(file).lines().map(|v| -> LineType {
		let line = v.expect("failed to read from input");

		if line.is_empty() {
			LineType::NewLine
		} else {
			let calorie = line.parse().expect("input contained invalid number");
			LineType::Calorie(calorie)
		}

	}).filter_map(|line| match line {
		LineType::NewLine => {
			if accumulator > 0 {
				let elf_value = accumulator;
				accumulator = 0;
				Some(elf_value)
			} else {
				None
			}
		},
		LineType::Calorie(calorie) => {
			accumulator = accumulator + calorie;
			None
		}
	}).for_each(|calories_per_elf| {
		for i in (0..3).rev() {
			if max_calories[i] >= calories_per_elf {
				break
			}

			max_calories[i + 1] = max_calories[i];
			max_calories[i] = calories_per_elf;
		}
	});
	
	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	let top_three = max_calories[0] + max_calories[1] + max_calories[2];
	println!("Result: {top_three}");
}
