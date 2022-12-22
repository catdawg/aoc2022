use std::cmp::Ordering;
use std::env;
use std::fs;
use std::iter::Peekable;
use std::str::CharIndices;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

	let now = std::time::Instant::now();

	enum InputType {
		OpenBracket,
		CloseBracket,
		Number(u32)
	}

	struct PacketParser<'a> {
		packet: &'a str,
		chars: Peekable<CharIndices<'a>>,
		virtual_closing_brackets: u32,
	}

	fn prepare_parser<'a>(packet: &'a str) -> PacketParser<'a> {
		PacketParser {packet, chars: packet.char_indices().peekable(), virtual_closing_brackets: 0}
	}

	impl<'a> Iterator for PacketParser<'a> {
		type Item = InputType;

		fn next(&mut self) -> Option<Self::Item> {
			loop {
				if self.virtual_closing_brackets > 0 {
					self.virtual_closing_brackets -= 1;
					return Some(InputType::CloseBracket);
				}
				let (start_index, c)  = match self.chars.next() {
					Option::None => return Option::None,
					Option::Some(v) => v
				};
	
				match c {
					'[' => {
						return Some(InputType::OpenBracket);
					},
					']' => {
						return Some(InputType::CloseBracket);
					},
					',' => {
						continue;
					}
					_ => {
						let mut end_index = start_index;
						loop {
							match self.chars.peek().unwrap() {
								(_, '[' | ']' | ',') => {
									let slice = &self.packet[start_index..=end_index];
									let parsed: u32 = slice.parse().unwrap();
									return Some(InputType::Number(parsed));
								},
								_ => {
									self.chars.next();
									end_index += 1;
								}
							}
						}
					}
				}
			}
		}
	}

	let divider_packet_1 = "[[2]]";
	let divider_packet_2 = "[[6]]";

	let mut packets: Vec<&str> = input.lines().filter(|s| !s.is_empty()).collect();

	packets.push(divider_packet_1);
	packets.push(divider_packet_2);

	packets.sort_by(|packet_1, packet_2| {
		let mut line1_parser = prepare_parser(packet_1);
		let mut line2_parser = prepare_parser(packet_2);
		
		let mut next1 = line1_parser.next();
		let mut next2 = line2_parser.next();

		loop {
			match (&next1, &next2) {
				(None, None) => {
					return Ordering::Equal;
				}
				(None, _) => {
					return Ordering::Less;
				},
				(_, None) => {
					return Ordering::Greater;
				},
				(Some(input1), Some(input2)) => {
					match (input1, input2) {
						(InputType::Number(input1_number), input2) => {
							match input2 {
								InputType::Number(input2_number) => {
									if input1_number < input2_number {
										return Ordering::Less;
									} else if input1_number > input2_number {
										return Ordering::Greater;
									}
									next1 = line1_parser.next();
									next2 = line2_parser.next();
								},
								InputType::OpenBracket => {
									line1_parser.virtual_closing_brackets += 1;
									next2 = line2_parser.next();
								},
								InputType::CloseBracket => {
									return Ordering::Greater;
								},
							}
						},
						(InputType::OpenBracket, input2) => {
							match input2 {
								InputType::Number(_) => {
									next1 = line1_parser.next();
									line2_parser.virtual_closing_brackets += 1;
								},
								InputType::OpenBracket => {
									next1 = line1_parser.next();
									next2 = line2_parser.next();
								},
								InputType::CloseBracket => {
									return Ordering::Greater;
								},
							}
						},
						(InputType::CloseBracket, input2) => {
							match input2 {
								InputType::Number(_) => {
									return Ordering::Less;
								},
								InputType::OpenBracket => {
									return Ordering::Less;
								},
								InputType::CloseBracket => {
									next1 = line1_parser.next();
									next2 = line2_parser.next();
								},
							}
						},
					}
				}
			}
		};
	});

	let mut res = 1;

	for (index, packet) in packets.iter().enumerate() {
		if *packet == divider_packet_1 || *packet == divider_packet_2 {
			res *= index + 1;
		}
	}

	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
		