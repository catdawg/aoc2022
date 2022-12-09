use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
		
	let now = std::time::Instant::now();

	let mut line_iter = input.lines().peekable();

	let num_cols = line_iter.peek().unwrap().len();

	let new_line_len = 1; // could be wrong, but it's ok
	let mut grid: Vec<u8> = Vec::with_capacity(num_cols * (input.len() / (num_cols + new_line_len)));

	for line in input.lines() {
		for char in line.chars() {
			grid.push(((char as u32) - ('0' as u32)).try_into().unwrap());
		}
	}
	
	let get_tree = |row: usize, col: usize| {
		grid.get(row * num_cols + col).unwrap()
	};

	let num_rows = grid.len() / num_cols;

	let res = grid.iter().enumerate().fold(0, |max, (index, tree)| {
		let col = index % num_cols;
		let row = index / num_cols;

		if row == 0 || row == (num_rows - 1) || col == 0 || col == (num_cols - 1) {
			return max
		}

		let mut score = 1;
		// check from top
		{
			let mut trees_found = 0;

			for new_row in (0..row).rev() {
				let next_tree = get_tree(new_row, col);
				trees_found += 1;

				if *next_tree >= *tree {
					break;
				}
			}

			score *= trees_found;
		}

		// check from bottom
		{
			let mut trees_found = 0;

			for new_row in (row + 1)..num_rows {
				let next_tree = get_tree(new_row, col);
				trees_found += 1;

				if *next_tree >= *tree {
					break;
				}
			}

			score *= trees_found;
		}
		
		// check from left
		{
			let mut trees_found = 0;

			for new_col in (0..col).rev() {
				let next_tree = get_tree(row, new_col);
				trees_found += 1;

				if *next_tree >= *tree {
					break;
				}
			}

			score *= trees_found;
		}

		// check from right
		{
			let mut trees_found = 0;

			for new_col in (col + 1)..num_cols {
				let next_tree = get_tree(row, new_col);
				trees_found += 1;

				if *next_tree >= *tree {
					break;
				}
			}

			score *= trees_found;
		}

		u32::max(score, max)
	});

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
