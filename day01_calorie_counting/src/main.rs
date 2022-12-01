use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct CalorieParserHelper<'a> {
	buf: String,
	buf_reader: BufReader<&'a File>,
}

impl<'a> Iterator for CalorieParserHelper<'a> {
	type Item = u32;

	fn next(&mut self) -> Option<u32> {

		let mut calorie_accumulator = 0;
		loop {
			self.buf.clear();
			let bytes_read = self.buf_reader.read_line(&mut self.buf).expect("failed to read from input");
			if bytes_read == 0 {
				if calorie_accumulator > 0 {
					break
				} else {
					return None;
				}
			}

			let trimmed_buf = self.buf.trim();
			if trimmed_buf.is_empty() {
				break
			}
			let num: u32 = trimmed_buf.parse().expect("input contained invalid number");

			calorie_accumulator += num
		}

		return Some(calorie_accumulator)
	}

}

fn prepare_parser(file: &File) -> CalorieParserHelper {
	let buf_reader = BufReader::new(file);
	let buf = String::new();

	CalorieParserHelper {buf, buf_reader}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let file = File::open(file_path).expect("Should have been able to read the file");

	let parser = prepare_parser(&file);

	let mut max_calorie = 0;

	for calorie in parser {
		max_calorie = max(calorie, max_calorie)
	}

	println!("max calorie:\n{max_calorie}");
}
