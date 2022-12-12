use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
		
	let now = std::time::Instant::now();

	#[derive(Copy, Clone)]
	enum Action {
		Left,
		Right,
		Up,
		Down
	}

	let iter = input.lines().map(|line| {
		let mut split = line.split(" ");
		(
			match split.next().unwrap() {
				"U" => Action::Up,
				"R" => Action::Right,
				"L" => Action::Left,
				"D" => Action::Down,
				_ => panic!()
			},
			split.next().unwrap().parse::<u32>().unwrap()
		)
	});

	// there is probably a better way to do this :(
	fn new_tail_pos(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {

		let x_diff = head_x - tail_x;
		let y_diff = head_y - tail_y;

		let x_diff_abs = i32::abs(x_diff);
		let y_diff_abs = i32::abs(y_diff);
		
		let x_diff_abs_1 = x_diff_abs == 1;
		let y_diff_abs_1 = y_diff_abs == 1;

		let x_diff_abs_greater_1 = x_diff_abs >= 2;
		let y_diff_abs_greater_1 = y_diff_abs >= 2;

		let move_x = x_diff_abs_1  && y_diff_abs_greater_1 || x_diff_abs_greater_1;
		let move_y = x_diff_abs_greater_1 && y_diff_abs_1 || y_diff_abs_greater_1;

		let delta_x = if move_x { i32::clamp(x_diff, -1, 1) } else { 0 };
		let delta_y = if move_y { i32::clamp(y_diff, -1, 1) } else { 0 };

		return (
			tail_x + delta_x, 
			tail_y + delta_y
		);
	}

	let mut visited = HashSet::new();
	
	let mut head_x = 0;
	let mut head_y = 0;

	let mut tail_x = 0;
	let mut tail_y = 0;

	for (action, count) in iter {

		match action {
			Action::Down => {
				for _ in 0..count {
					head_y -= 1;
					(tail_x, tail_y) = new_tail_pos(head_x, head_y, tail_x, tail_y);
					visited.insert((tail_x, tail_y));
				}
			},
			Action::Left => {
				for _ in 0..count {
					head_x -= 1;
					(tail_x, tail_y) = new_tail_pos(head_x, head_y, tail_x, tail_y);
					visited.insert((tail_x, tail_y));
				}
			},
			Action::Right => {
				for _ in 0..count {
					head_x += 1;
					(tail_x, tail_y) = new_tail_pos(head_x, head_y, tail_x, tail_y);
					visited.insert((tail_x, tail_y));
				}
			},
			Action::Up =>  {
				for _ in 0..count {
					head_y += 1;
					(tail_x, tail_y) = new_tail_pos(head_x, head_y, tail_x, tail_y);
					visited.insert((tail_x, tail_y));
				}
			},
		}
	}

	let res = visited.len();
	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
