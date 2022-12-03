use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let file = File::open(file_path).expect("Should have been able to read the file");

	let now = std::time::Instant::now();

	fn get_priority(item_type: char) -> u32 {
		match item_type {
			'a'..='z' => (item_type as u32) - ('a' as u32) + 1,
			'A'..='Z' => (item_type as u32) - ('A' as u32) + 27,
			_ => 0
		}
	}

	let mut set = [false; 27 * 2];
	let res = BufReader::new(file).lines().fold(0, |state, res| {

		match res {
			Ok(v) => {
				let half_length = v.len() / 2;
				let mut shared_item_priority = 0;
				for (i, c) in v.chars().enumerate() {

					let priority = get_priority(c);
					if i < half_length {
						set[priority as usize] = true;
					} else {
						if set[priority as usize] {
							shared_item_priority = priority;
							break;
						}
					}
				}

				set.fill(false);
				state + shared_item_priority
			},
			Err(_) => state
		}
	});
	
	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
