use std::env;
use std::fs;
use std::iter::zip;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
		
	let now = std::time::Instant::now();

	let iter = zip(
		input.chars(), 
		zip(
			input.chars().skip(1), 
			zip(
				input.chars().skip(2), 
				input.char_indices().skip(3)
			)
		)
	);

	let mut res: usize = 0;
	for (c1, (c2, (c3, (index, c4)))) in iter {
		if c1 != c2 && c1 != c3 && c1 != c4 && c2 != c3 && c2 != c4 && c3 != c4 {
			res = index + 1;
			break;
		}
	};

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
