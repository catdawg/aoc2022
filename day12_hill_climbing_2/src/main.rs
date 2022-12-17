use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

	let now = std::time::Instant::now();

	enum NodeType {
		Start,
		End,
		Height(u8),
	}

	struct Node {
		col: usize,
		row: usize,
		node_type: NodeType,
		connections: [Option<(usize, usize)>; 4],
		distance: u32,
		visited: bool
	}

	let mut nodes: Vec<Node> = vec![];

	let row_length = input.lines().enumerate().peekable().peek().unwrap().1.len();

	fn try_link(node: &mut Node, next: &mut Node) {
		fn connect(node: &mut Node, next: (usize, usize)) {
			for connection in node.connections.iter_mut() {
				match connection {
					None => {
						*connection = Some(next);
						break;
					}
					_ => {}
				}
			}
		}

		let node_height = match node.node_type {
			NodeType::Start => {
				'a' as u8
			}, 
			NodeType::End => {
				'z' as u8
			},
			NodeType::Height(height) => {
				height
			},
		};
		let next_height = match next.node_type {
			NodeType::Start => {
				'a' as u8
			}, 
			NodeType::End => {
				'z' as u8
			},
			NodeType::Height(height) => {
				height
			},
		};

		if next_height as i32 - node_height as i32 <= 1 {
			connect(node, (next.col, next.row));
		}
		
		if node_height as i32 - next_height as i32 <= 1 {
			connect(next, (node.col, node.row));
		}
	}

	fn index(row_length: usize, col: usize, row: usize) -> usize {
		row * row_length + col
	}

	for (row, line) in input.lines().enumerate() {
		for (col, height) in line.chars().enumerate() {
			let node_type = match height {
				'a' | 'S' => NodeType::Start,
				'E' => NodeType::End,
				c => NodeType::Height(c as u8),
			};

			let distance = if let NodeType::Start = node_type { 0 } else { u32::MAX };

			let mut node = Node {
				col: col,
				row: row,
				node_type: node_type,
				connections: [None; 4],
				distance: distance,
				visited: false
			};

			//left
			if col > 0 {
				let left_node = nodes.get_mut(index(row_length, col - 1, row)).unwrap();
				try_link(&mut node, left_node)
			}

			//top
			if row > 0 {
				let top_node = nodes.get_mut(index(row_length, col, row - 1)).unwrap();
				try_link(&mut node, top_node)
			}

			nodes.push(node);
		}
	}

	let start_nodes: Vec<(usize, usize)> =
		nodes.iter()
		.filter(|n| {match n.node_type {NodeType::Start => true, _ => false}})
		.map(|n| (n.col, n.row))
		.collect();

	let res = start_nodes.iter().fold(u32::MAX, |acc, (start_node_col, start_node_row)| {
		for node in nodes.iter_mut() {
			node.distance = u32::MAX;
			node.visited = false;
		}
		let mut unvisited: Vec<(usize, usize)> = nodes.iter().map(|node| (node.col, node.row)).rev().collect();
		
		let start_node = nodes.get_mut(index(row_length, *start_node_col, *start_node_row)).unwrap();
		start_node.distance = 0;

		while unvisited.len() > 0 {
			unvisited.sort_by(|(a_col, a_row), (b_col, b_row)| {
				let node_a = nodes.get(index(row_length, *a_col, *a_row)).unwrap();
				let node_b = nodes.get(index(row_length, *b_col, *b_row)).unwrap();
				node_b.distance.cmp(&node_a.distance)
			});
	
			let (node_col, node_row) = unvisited.pop().unwrap();
			let node = nodes.get_mut(index(row_length, node_col, node_row)).unwrap();
			
			node.visited = true;
	
			if let NodeType::End = node.node_type {
				break;
			}
	
			let node_distance = node.distance;
	
			for i in 0..4 {
				let node = nodes.get_mut(index(row_length, node_col, node_row)).unwrap();
				let connection = node.connections[i];
				match connection {
					Some((next_col, next_row)) => {
						let next_node = nodes.get_mut(index(row_length, next_col, next_row)).unwrap();
	
						if !next_node.visited {
							next_node.distance = next_node.distance.min(node_distance + 1);
						}
					}
					None => break,
				}
			}
		}
	
		let end_node = nodes
			.iter()
			.find(|node| {
				if let NodeType::End = node.node_type {
					true
				} else {
					false
				}
			})
			.unwrap();

		acc.min(end_node.distance)
	});

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
		