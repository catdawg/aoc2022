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
		if first_start == second_start || first_end == second_end || first_start == second_end || second_start == first_end {
			true
		} else if first_start < second_start {
			first_end > second_start
		} else {
			second_end > first_start
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
	});
	
	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
