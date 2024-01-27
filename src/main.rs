mod model;
mod puzzle_solver;

use clap::Parser;

use crate::{model::PuzzleInput, puzzle_solver::solve_word_star_puzzle};

/// VGs word star puzzle solver
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The required letter
    #[arg(short, long)]
    required_character: char,

    /// A list of allowed letters
    #[arg(short, long, num_args = 7, value_delimiter = ' ')]
    allowed_characters: Vec<char>,
}

fn main() {
    let args = Args::parse();

    let puzzle_input = PuzzleInput::new(args.required_character, args.allowed_characters);

    let result = solve_word_star_puzzle(puzzle_input);

    println!("{:?}", result)
}
