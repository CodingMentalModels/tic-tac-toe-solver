use crate::board::board::{Board, Move, Outcome, Player};
use crate::tree::tree::{Tree, Node};

pub struct Solver {
    tree: Tree,
}

impl Solver {
    pub fn from_board(board: Board) -> Self {
        let tree = Tree::from_board(board);
        Solver { tree }
    }

    pub fn get_evaluation(&self) -> f32 {
        let root = self.tree.get_root();
        self.get_evaluation_for_node(root)
    }

    fn get_evaluation_for_node(&self, node: &Node) -> f32 {
        let children = node.get_children().iter();
        if children.len() == 0 {
            return Solver::get_evaluation_for_outcome(node.get_board().get_outcome());
        }
        let active_player = node.get_active_player();
        let mut best_move = match active_player {
            Some(Player::X) => -1.,
            Some(Player::O) => 1.,
            None => panic!("There's no active player even though there the node has children."),
        };
        for child in children {
            let child_evaluation = self.get_evaluation_for_node(child);
            match active_player {
                Some(Player::X) => {
                    if child_evaluation > best_move {
                        best_move = child_evaluation;
                    }
                },
                Some(Player::O) => {
                    if child_evaluation < best_move {
                        best_move = child_evaluation;
                    }
                },
                None => panic!("There's no active player even though there the node has children."),
            }
        }
        return best_move;
    }

    fn get_evaluation_for_outcome(outcome: Outcome) -> f32 {
        match outcome {
            Outcome::InProgress => 0.,
            Outcome::Ambiguous => 0.,
            Outcome::Draw => 0.,
            Outcome::Victory(Player::X) => 1.,
            Outcome::Victory(Player::O) => -1.,
        }
    }

    pub fn get_next_moves(&mut self) -> Result<Vec<Move>, String> {
        let (next_moves, _) = self.get_next_moves_and_evaluation()?;
        return Ok(next_moves);
    }

    pub fn get_next_moves_and_evaluation(&mut self) -> Result<(Vec<Move>, f32), String> {

        let active_player = match self.tree.get_root().get_active_player() {
            Some(player) => player,
            None => return Err("The game is already over.".to_string()),
        };

        let root = self.tree.get_root();
        let mut next_moves = Vec::new();
        let mut best_evaluation = match active_player {
            Player::X => -2.,
            Player::O => 2.,
        };

        for m in root.get_legal_moves().iter() {
            let child = root.get_child(m.get_row(), m.get_column()).unwrap();
            let evaluation = self.get_evaluation_for_node(child);
            if ((evaluation > best_evaluation) && (active_player == Player::X)) ||
                ((evaluation < best_evaluation) && (active_player == Player::O)) {
                best_evaluation = evaluation;
                next_moves.clear();
                next_moves.push(*m);
            } else if evaluation == best_evaluation {
                next_moves.push(*m);
            }
        }
        return Ok((next_moves, best_evaluation));
    }
}


#[cfg(test)]
mod test_solver {
    use super::*;

    #[test]
    fn test_solver_gets_evaluation() {
        assert_eq!(
            Solver::from_board(
                Board::from_position(
                    "XOX
                    OXO
                    XOX",
                ).unwrap()
            ).get_evaluation(),
            1.
        );
        
        assert_eq!(
            Solver::from_board(
                Board::from_position(
                    "XOX
                    O_O
                    XOX",
                ).unwrap()
            ).get_evaluation(),
            1.
        );

        assert_eq!(
            Solver::from_board(
                Board::from_position(
                    "XOX
                    OO_
                    XXO",
                ).unwrap()
            ).get_evaluation(),
            0.
        );

        assert_eq!(
            Solver::from_board(
                Board::from_position(
                    "XO_
                    OO_
                    XXO",
                ).unwrap()
            ).get_evaluation(),
            -1.
        );

        
        assert!(
            Solver::from_board(
                Board::from_position(
                    "XO_
                    O__
                    XXO",
                ).unwrap()
            ).get_evaluation() < 0.0001
        );
    }
}