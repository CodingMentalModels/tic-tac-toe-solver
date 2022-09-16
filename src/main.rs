mod board;
mod tree;
mod solver;

use clap::{App, SubCommand, Arg};
use crate::board::board::{Board, Move};
use crate::solver::solver::Solver;



fn main() {
    let matches = App::new("Tic Tac Toe Solver")
		.about("Solver for Tic Tac Toe")
		.subcommand(
			SubCommand::with_name("solve")
				.about("Solve Tic Tac Toe Position")
				.arg(
					Arg::with_name("Position")
						.help("Tic Tac Toe Position")						
				)
            ).get_matches();
    
    if let Some(matches) = matches.subcommand_matches("solve") {
        match matches.value_of("Position") {
            Some(position) => {
                match Board::from_position(position) {
                    Ok(board) => {
                        let mut solver = Solver::from_board(board);
                        let next_moves = solver.get_next_moves();
                        println!("Next moves: {:?}", next_moves);
                    },
                    Err(error) => {
                        println!("Invalid Position: {}", error);
                    }
                }
            },
            None => {
                println!("Needs a Position!");
            }
        }
    } else {
        println!("Invalid command!");
    }
}

#[cfg(test)]
mod test_integration_tests {
    use super::*;

    #[test]
    fn test_solver_solves_tic_tac_toe() {
        let mut solver = Solver::from_board(
            Board::from_position(
                "XOX
                O_O
                XOX",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves(), Ok(vec![Move::new(1, 1)]));

        let mut solver = Solver::from_board(
            Board::from_position(
                "X_X
                O_O
                ___",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves(), Ok(vec![Move::new(0, 1), Move::new(1, 1)]));

        let mut solver = Solver::from_board(
            Board::from_position(
                "XO_
                _X_
                ___",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves().unwrap().len(), 6);
        
        let mut solver = Solver::from_board(
            Board::from_position(
                "XO_
                _X_
                __O",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves_and_evaluation(), Ok((vec![Move::new(1, 0), Move::new(2, 0)], 1.)));

        let mut solver = Solver::from_board(
            Board::from_position(
                "XO_
                ___
                ___",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves_and_evaluation(), Ok((vec![Move::new(1, 0), Move::new(1, 1), Move::new(2, 0)], 1.)));

        
        let mut solver = Solver::from_board(
            Board::from_position(
                "XOX
                _O_
                __X",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves_and_evaluation(), Ok((vec![Move::new(1, 2), Move::new(2, 1)], -1.)));

        let mut solver = Solver::from_board(
            Board::from_position(
                "XO_
                O__
                XXO",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves_and_evaluation(), Ok((vec![Move::new(0, 2), Move::new(1, 1), Move::new(1, 2)], 0.)));

    }

}