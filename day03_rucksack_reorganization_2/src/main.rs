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

	#[derive(Copy, Clone)]
	enum RucksackPresence {
		NotPresent,
		PresentInFirst,
		PresentInFirstAndSecond
	}
	
	#[derive(Copy, Clone)]
	enum Elf {
		First,
		Second,
		Third
	}

	let mut set = [RucksackPresence::NotPresent; 27 * 2];
	let (_, res) = BufReader::new(file).lines().fold((Elf::First, 0), |(elf, accumulator), res| {

		match res {
			Ok(v) => {
				
				let mut priority_of_shared_item_type = 0;
				match elf {
					Elf::First => {
						for c in v.chars() {

							let priority = get_priority(c);
							set[priority as usize] = RucksackPresence::PresentInFirst;
						}
					},
					Elf::Second => {
						for c in v.chars() {

							let priority = get_priority(c);

							match set[priority as usize] {
								RucksackPresence::PresentInFirst => {
									set[priority as usize] = RucksackPresence::PresentInFirstAndSecond
								},
								_ => {}					
							};
						}
					},
					Elf::Third => {
						for c in v.chars() {

							let priority = get_priority(c);

							match set[priority as usize] {
								RucksackPresence::PresentInFirstAndSecond => {
									priority_of_shared_item_type = priority;
									break;
								},
								_ => {}					
							};
						}
					},
				};

				let new_elf = match elf {
					Elf::First => Elf::Second,
					Elf::Second => Elf::Third,
					Elf::Third => Elf::First,
				};

				match new_elf {
					Elf::First => set.fill(RucksackPresence::NotPresent),
					_ => {}
				};

				(new_elf, accumulator + priority_of_shared_item_type)
			},
			Err(_) => (elf, accumulator)
		}
	});
	
	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
