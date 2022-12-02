use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let file = File::open(file_path).expect("Should have been able to read the file");

	let now = std::time::Instant::now();

	#[derive(Copy, Clone)]
	enum Move {
		Rock,
		Paper,
		Scissors
	}

	fn score (other: Move, mine: Move) -> u32 {
		match other {
			Move::Rock => match mine {
				Move::Rock => 3 + 1,
				Move::Paper => 6 + 2,
				Move::Scissors => 0 + 3
			},
			Move::Paper => match mine {
				Move::Rock => 0 + 1,
				Move::Paper => 3 + 2,
				Move::Scissors => 6 + 3
			},
			Move::Scissors => match mine {
				Move::Rock => 6 + 1,
				Move::Paper => 0 + 2,
				Move::Scissors => 3 + 3
			}
		}
	}

	let (_, res) = BufReader::new(file).bytes().fold((Move::Rock, 0), |state, res| {
		match res {
			Ok(v) => {
				match v as char {
					'A' => (Move::Rock, state.1),
					'B' => (Move::Paper, state.1),
					'C' => (Move::Scissors, state.1),
					'X' => (state.0, state.1 + score(state.0, Move::Rock)),
					'Y' => (state.0, state.1 + score(state.0, Move::Paper)),
					'Z' => (state.0, state.1 + score(state.0, Move::Scissors)),
					_ => state
				}
			},
			Err(_) => state
		}
	});
	
	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
