mod board;
mod tree;
mod solver;

use crate::board::board::{Board, Move};
use crate::solver::solver::Solver;

fn main() {
    println!("Hello, world!");
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
        assert_eq!(solver.get_next_moves(), vec![Move::new(1, 1)]);

        let mut solver = Solver::from_board(
            Board::from_position(
                "X_X
                O_O
                ___",
            ).unwrap()
        );
        assert_eq!(solver.get_next_moves(), vec![Move::new(0, 1)]);
    }

}