use crate::board::{Board, Cell};
use crate::pos::Pos;
use crate::piece::Piece;

use itertools::Itertools;

type Solution = Vec<(Piece, Pos)>;

pub fn solve_with_goals(mut board: Board, pieces: &[Vec<Piece>], goal1: Pos, goal2: Pos) -> Vec<Solution> {
    board[goal1] = Cell::Goal;
    board[goal2] = Cell::Goal;
    solve(board, pieces)
}

fn solve(mut board: Board, pieces: &[Vec<Piece>]) -> Vec<Solution> {
    let mut solutions = Vec::new();
    let mut solution = Vec::new();
    for piece_order in pieces.iter().permutations(pieces.len()) {
        solve_with_piece_order(
            &mut board,
            &piece_order,
            Pos::new(0, 0),
            &mut solution,
            &mut solutions);
    }
    solutions
}

fn solve_with_piece_order(
    board: &mut Board,
    pieces: &[&Vec<Piece>],
    pos: Pos,
    solution: &mut Solution,
    solutions: &mut Vec<Solution>) {
    if !board[pos].is_empty() {
        if pos.x + 1 == board.width {
            if pos.y + 1 == board.height {
                debug_assert!(pieces.is_empty());
                solutions.push(solution.clone());
            } else {
                solve_with_piece_order(
                    board, pieces, Pos::new(0, pos.y + 1), solution, solutions);
            }
        } else {
            solve_with_piece_order(
                board, pieces, Pos::new(pos.x + 1, pos.y), solution, solutions);
        }
        return;
    }
    debug_assert!(!pieces.is_empty());
    let curr_piece = pieces[0];
    let pieces = &pieces[1..];
    for piece in curr_piece {
        let piece_pos = Pos::new(pos.x - piece.x_offset(), pos.y);
        if board.can_place(&piece, piece_pos) {
            board.place(&piece, piece_pos);
            solution.push((piece.clone(), piece_pos));
            solve_with_piece_order(board, pieces, pos, solution, solutions);
            solution.pop();
            board.unplace(&piece, piece_pos);
        }
    }
}
