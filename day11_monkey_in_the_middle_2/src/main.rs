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
		Add(u64),
		Mul(u64),
		Pow2
	}

	#[derive(Copy, Clone)]
	struct Test {
		divisible_by: u64,
		success: usize,
		fail: usize,
	}

	struct Monkey {
		items: Vec<u64>,
		op: Op,
		test: Test
	}

	fn apply_test(num: u64, test: Test) -> usize {
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
								.parse::<u64>().unwrap()
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
						let divisible_by: u64 = divisible_by_line
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

	let common_multiple = monkeys.iter().fold(1, |acc, m| {m.test.divisible_by * acc});

	let mut passed_items: Vec<(usize, u64)> = vec![];
	let mut monkey_inspection_times: Vec<u64> = vec![0; monkeys.len()];
 	for _ in 0..10000 {
		for current_monkey_index in 0..monkey_count {
			let current_monkey = monkeys.get_mut(current_monkey_index).unwrap();
	
			let current_monkey_op = current_monkey.op;
			let current_monkey_test = current_monkey.test;
			let current_monkey_items = &mut current_monkey.items;

			for mut item in current_monkey_items.drain(..) {
				
				match current_monkey_op {
					Op::Add(arg) => item += arg,
					Op::Mul(arg) => item *= arg,
					Op::Pow2 => {
						item *= item;
					},
				};
				
				let next_monkey = apply_test(item, current_monkey_test);
				item = item % common_multiple;

				*monkey_inspection_times.get_mut(current_monkey_index).unwrap() += 1;
				passed_items.push((next_monkey, item));
			}

			for (monkey, item) in passed_items.drain(..) {
				let current_monkey = monkeys.get_mut(monkey).unwrap();
				current_monkey.items.push(item);
			}
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
