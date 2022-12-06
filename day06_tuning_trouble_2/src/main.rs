use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
		
	let now = std::time::Instant::now();

	let mut set: [Option<u64>; 27] = [None; 27];

	let mut global_index: usize = 0;
	let mut search_id: u64 = 0; 
	while global_index < input.len() {

		let sub_string = input.get(global_index..global_index + 14).unwrap();

		let mut success = true;
		for (sub_index, char) in sub_string.char_indices() {

			let set_index = char as usize - ('a' as usize);
			let set_lookup = set.get(set_index).unwrap();

			match set_lookup {
				Some(prev_index) => {
					if *prev_index >= search_id {
						//fail

						let diff = *prev_index - search_id;
						global_index += diff as usize;

						success = false;

						break;
					} else {
						set[set_index] = Some(search_id + sub_index as u64);
					}
				},
				None => {
					set[set_index] = Some(search_id + sub_index as u64);
				}
			}
		}

		if success {
			global_index += 14;
			break;
		}

		global_index += 1;
		search_id += 14;
	}

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {global_index}");
}
