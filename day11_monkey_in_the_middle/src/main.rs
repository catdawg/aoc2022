use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
		
	let now = std::time::Instant::now();

	#[derive(Copy, Clone)]
	enum Op {
		Add(i64),
		Mul(i64),
		Pow2
	}

	#[derive(Copy, Clone)]
	struct Test {
		divisible_by: i64,
		success: usize,
		fail: usize,
	}

	struct Monkey {
		items: Vec<i64>,
		op: Op,
		test: Test
	}

	fn apply_op(op: Op, num: i64) -> i64 {
		match op {
			Op::Add(arg) => num + arg,
			Op::Mul(arg) => num * arg,
			Op::Pow2 => num * num,
		}
	}

	fn apply_base_op(num: i64) -> i64 {
		f64::floor((num as f64) / 3.0) as i64
	}

	fn apply_test(num: i64, test: Test) -> usize {
		if num % test.divisible_by == 0 {
			test.success
		} else {
			test.fail
		}
	}

	let mut monkeys: Vec<Monkey> = vec!();
	let mut iter = input.lines();

	while let Some(line) = iter.next() {
		let mut split = line.split(" ").filter(|s| s.trim().len() > 0);
		match split.next() {
			Some(s) => {
				match s {
					"Monkey" => {
						let starting_items_line = iter.next().unwrap();

						let items = 
							starting_items_line
							.split(":")
							.skip(1)
							.next().unwrap()
							.split(",")
							.map(|s| 
								s.trim()
								.parse::<i64>().unwrap()
							);

						let operation_line = iter.next().unwrap();
						let mut op_split = operation_line
							.split(":")
							.skip(1)
							.next().unwrap()
							.split("=")
							.skip(1)
							.next().unwrap()
							.split(" ")
							.filter_map(|s| {
									let trimmed_s = s.trim();
									match trimmed_s.len() {
										0 => None,
										_ => Some(trimmed_s)
									}
								}
							)
							.skip(1);

						let op = match op_split.next().unwrap() {
							"*" => {
								match op_split.next().unwrap() {
									"old" => {
										Op::Pow2
									},
									s => {
										Op::Mul(s.parse().unwrap())
									}
								}
							},
							"+" => {
								match op_split.next().unwrap() {
									s => {
										Op::Add(s.parse().unwrap())
									}
								}
							},
							_ => {panic!()}
						};
						
						let divisible_by_line = iter.next().unwrap();
						let divisible_by: i64 = divisible_by_line
							.split("by")
							.skip(1)
							.map(|s| s.trim())
							.next().unwrap()
							.parse().unwrap();

						let monkey_true_line = iter.next().unwrap();
						let monkey_true: usize = monkey_true_line
							.split("monkey")
							.skip(1)
							.map(|s| s.trim())
							.next().unwrap()
							.parse().unwrap();

						let monkey_false_line = iter.next().unwrap();
						let monkey_false: usize = monkey_false_line
							.split("monkey")
							.skip(1)
							.map(|s| s.trim())
							.next().unwrap()
							.parse().unwrap();

						monkeys.push(Monkey {
							items: items.collect(),
							op: op,
							test: Test {
								divisible_by,
								success: monkey_true,
								fail: monkey_false
							}
						})
					},
					_ => {}
				}
			},
			None => {}
		}
	}

	let monkey_count = monkeys.len();

	let mut passed_items: Vec<(usize, i64)> = vec![];
	let mut monkey_inspection_times: Vec<u32> = vec![0; monkeys.len()];
 	for _ in 0..20 {
		for current_monkey_index in 0..monkey_count {
			let current_monkey = monkeys.get_mut(current_monkey_index).unwrap();
	
			let current_monkey_op = current_monkey.op;
			let current_monkey_test = current_monkey.test;
			let current_monkey_items = &mut current_monkey.items;

			current_monkey_items.into_iter().map(|item| {
				let new_item_value = apply_base_op(apply_op(current_monkey_op, *item));
				let next_monkey = apply_test(new_item_value, current_monkey_test);

				*monkey_inspection_times.get_mut(current_monkey_index).unwrap() += 1;

				(next_monkey, new_item_value)
			}).for_each(|monkey_and_item| {
				passed_items.push(monkey_and_item);
			});
			current_monkey_items.clear();
			
			for (monkey, item) in &passed_items {
				let current_monkey = monkeys.get_mut(*monkey).unwrap();
				current_monkey.items.push(*item);
			}	
			passed_items.clear();		
		}
	}

	monkey_inspection_times.sort();
	
	let res = 
		monkey_inspection_times.get(monkey_inspection_times.len() - 1).unwrap() * 
		monkey_inspection_times.get(monkey_inspection_times.len() - 2).unwrap();

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
