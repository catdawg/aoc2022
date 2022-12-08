use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();

	let file_path = &args[1];

	let file = fs::read_to_string(file_path).unwrap();

	let now = std::time::Instant::now();

	#[derive(Copy, Clone)]
	enum Move {
		Rock = 1,
		Paper = 2,
		Scissors = 3
	}

	#[derive(Copy, Clone)]
	enum Outcome {
		Lose = 0,
		Draw = 3,
		Win = 6
	}

	fn score (other: Move, outcome: Outcome) -> u32 {
		match other {
			Move::Rock => match outcome {
				Outcome::Lose => Outcome::Lose as u32 + Move::Scissors as u32,
				Outcome::Draw => Outcome::Draw as u32 + Move::Rock as u32,
				Outcome::Win => Outcome::Win as u32 + Move::Paper as u32
			},
			Move::Paper => match outcome {
				Outcome::Lose => Outcome::Lose as u32 + Move::Rock as u32,
				Outcome::Draw => Outcome::Draw as u32 + Move::Paper as u32,
				Outcome::Win => Outcome::Win as u32 + Move::Scissors as u32
			},
			Move::Scissors => match outcome {
				Outcome::Lose => Outcome::Lose as u32 + Move::Paper as u32,
				Outcome::Draw => Outcome::Draw as u32 + Move::Scissors as u32,
				Outcome::Win => Outcome::Win as u32 + Move::Rock as u32
			}
		}
	}

	let (_, res) = file.chars().fold((Move::Rock, 0), |state, res| {
		match res {
			'A' => (Move::Rock, state.1),
			'B' => (Move::Paper, state.1),
			'C' => (Move::Scissors, state.1),
			'X' => (state.0, state.1 + score(state.0, Outcome::Lose)),
			'Y' => (state.0, state.1 + score(state.0, Outcome::Draw)),
			'Z' => (state.0, state.1 + score(state.0, Outcome::Win)),
			_ => state
		}
	});
	
	let elapsed = now.elapsed();
	println!("Elapsed: {:.2?}", elapsed);

	println!("Result: {res}");
}
