use std::env;
use std::fs::read_to_string;
use std::iter::zip;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = read_to_string(file_path).unwrap();

	let now = std::time::Instant::now();

	fn get_priority(item_type: char) -> u32 {
		match item_type {
			'a'..='z' => (item_type as u32) - ('a' as u32) + 1,
			'A'..='Z' => (item_type as u32) - ('A' as u32) + 27,
			_ => 0
		}
	}

	#[derive(Copy, Clone)]
	enum RucksackPresence {
		NotPresent,
		PresentInFirst,
		PresentInFirstAndSecond
	}

	let mut set = [RucksackPresence::NotPresent; 27 * 2];
	let res = 
		zip(
			input.lines().step_by(3),
			zip(
				input.lines().skip(1).step_by(3),
				input.lines().skip(2).step_by(3)
			)
		).fold(
			0, |accumulator, (elf1, (elf2, elf3))| {

				
				let mut priority_of_shared_item_type = 0;
				for c in elf1.chars() {

					let priority = get_priority(c);
					set[priority as usize] = RucksackPresence::PresentInFirst;
				}
				
				for c in elf2.chars() {


					let priority = get_priority(c);

					match set[priority as usize] {
						RucksackPresence::PresentInFirst => {
							set[priority as usize] = RucksackPresence::PresentInFirstAndSecond
						},
						_ => {}					
					};
				}
				
				for c in elf3.chars() {

					let priority = get_priority(c);

					match set[priority as usize] {
						RucksackPresence::PresentInFirstAndSecond => {
							priority_of_shared_item_type = priority;
							break;
						},
						_ => {}					
					};
				}

				set.fill(RucksackPresence::NotPresent);

				accumulator + priority_of_shared_item_type
			}
		);
	
	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
