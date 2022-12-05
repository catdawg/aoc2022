use std::cell::RefCell;
use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
		
	let now = std::time::Instant::now();

	let mut lines_iter = input.lines();

	let mut stacks: Vec<RefCell<Vec<char>>> = vec![];

	for line in lines_iter.by_ref() {
		if line.is_empty() {
			break; // reached end of initial stack state
		}
		for (stack_index, crate_in_stack) in line.as_bytes().chunks(4).enumerate() {
			match (crate_in_stack[0] as char, crate_in_stack[1] as char, crate_in_stack[2] as char) {
				(' ', ' ', _) => {

				},
				('[', crate_name, _) => {
					let current_stack_size = stacks.len();

					if stack_index + 1 > current_stack_size {
						let missing_stacks = (stack_index + 1) - current_stack_size;
						for _ in 0..missing_stacks {
							stacks.push(RefCell::new(vec![]));
						}
					}

					stacks.get(stack_index).unwrap().borrow_mut().push(crate_name);
				},
				_ => {}
			}
		}
	}

	stacks.iter_mut().for_each(|reversed_stack| {reversed_stack.borrow_mut().reverse()});

	for line in lines_iter.by_ref() {
		let mut line_split = line.split(" ");
		line_split.next(); // "move"
		let move_count: u32 = line_split.next().unwrap().parse().unwrap();
		line_split.next(); // "from"
		let from_stack: usize = line_split.next().unwrap().parse().unwrap();
		line_split.next(); // "to"
		let to_stack: usize = line_split.next().unwrap().parse().unwrap();

		let mut from_stack_vec = stacks.get(from_stack - 1).unwrap().borrow_mut();
		let mut to_stack_vec = stacks.get(to_stack - 1).unwrap().borrow_mut();

		
		let from_stack_vec_len = from_stack_vec.len();
		for crate_index in 0..move_count {
			to_stack_vec.push(*from_stack_vec.get(from_stack_vec_len - (move_count as usize) + (crate_index as usize)).unwrap());
		}
		for _ in 0..move_count {
			from_stack_vec.pop();
		}
	}

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	let res = stacks.iter_mut().map(|stack| {
		*stack.borrow().last().unwrap()
	}).collect::<String>();
	println!("Result: {res}");
}
