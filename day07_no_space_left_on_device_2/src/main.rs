use std::cmp;
use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
		
	let now = std::time::Instant::now();

	struct Directory {
		name: String,
		size: u32,
		directory_indexes: Vec<usize>,
		parent_index: Option<usize>
	}

	let mut file_tree: Vec<Directory> = vec![];

	file_tree.push(Directory {
		name: String::from("/"),
		size: 0,
		directory_indexes: vec![],
		parent_index: None
	});


	let mut current_index = 0;

	for line in input.lines() {
		let mut split = line.split(" ");

		let first_token = split.next().unwrap();

		match first_token {
			"$" => {
				match split.next().unwrap() {
					"cd" => {
						let cd_arg = split.next().unwrap();

						match cd_arg {
							".." => {
								let current_directory = file_tree.get_mut(current_index).unwrap();
								current_index = current_directory.parent_index.unwrap();
							},
							"/" => {
								current_index = 0;
							},
							_ => {
								let current_directory = file_tree.get(current_index).unwrap();

								for directory_index in current_directory.directory_indexes.iter() {
									let sub_directory = file_tree.get(*directory_index).unwrap();

									if sub_directory.name == cd_arg {
										current_index = *directory_index;
										break;
									}
								}
							}
						}
					},
					_ => {}
				}
			}
			"dir" => {
				let new_index = file_tree.len();
				file_tree.push(Directory {
					name: String::from(split.next().unwrap()),
					size: 0,
					directory_indexes: vec![],
					parent_index: Some(current_index)

				});
				let current_directory = file_tree.get_mut(current_index).unwrap();
				current_directory.directory_indexes.push(new_index);
			},
			_ => {
				let size: u32 = first_token.parse().unwrap();

				let mut propagate_index_opt = Some(current_index);
				loop {
					match propagate_index_opt {
						Some (propagate_index) => {
							let current_directory = file_tree.get_mut(propagate_index).unwrap();
							current_directory.size += size;

							propagate_index_opt = current_directory.parent_index;
						},
						None => {
							break;
						}
					}
				}
			}
		}
	}

	let total_used_space = file_tree.get(0).unwrap().size;

	let minimum_needed = 30000000 - (70000000 - total_used_space);

	let mut candidate: u32 = u32::MAX;

	for dir in file_tree.iter() {
		if  dir.size >= minimum_needed {
			candidate = cmp::min(candidate, dir.size);
		} 
	}

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {candidate}");
}
