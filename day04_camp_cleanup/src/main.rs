use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let file = File::open(file_path).expect("Should have been able to read the file");

	let now = std::time::Instant::now();

	fn is_overlap(first_start: u32, first_end: u32, second_start: u32, second_end: u32) -> bool {
		if first_start == second_start || first_end == second_end {
			true
		} else if first_start < second_start {
			second_end < first_end
		} else {
			first_end < second_end
		}
	}

	let res = BufReader::new(file).lines().fold(0, |accumulator: u32, ranges| {
		match ranges {
			Ok(ranges) => {
				let mut iter = ranges.split(",").flat_map(|range| range.split("-"));
				let first_start: u32 = iter.next().expect("wrong file structure").parse().expect("wrong file structure");
				let first_end = iter.next().expect("wrong file structure").parse().expect("wrong file structure");
				let second_start = iter.next().expect("wrong file structure").parse().expect("wrong file structure");
				let second_end = iter.next().expect("wrong file structure").parse().expect("wrong file structure");

				if is_overlap(first_start, first_end, second_start, second_end) {
					accumulator + 1
				} else {
					accumulator
				}
			},
			Err(_) => accumulator
		}

		/* 

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
		} */
	});
	
	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
