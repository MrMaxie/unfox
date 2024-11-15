use std::{collections::HashSet, vec};

use serde::Serialize;

use crate::board::Board;

#[derive(Debug, Clone, Serialize)]
struct HistoryEntry {
    board: Board,
    pawn_at_move_to: Option<(u8, u8, u8)>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Solution {
    steps: Vec<HistoryEntry>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Branch {
    history: Vec<HistoryEntry>,
    board: Board,
}

struct Solver {
    branches: Vec<Branch>,
    solution: Option<Branch>,
    remaining_tries: usize,
}

impl Solver {
    fn new(board: Board, max_tries: usize) -> Self {
        let branches = vec![Branch {
            history: vec![
                HistoryEntry {
                    board: board.clone(),
                    pawn_at_move_to: None,
                },
            ],
            board,
        }];

        Self {
            branches,
            solution: None,
            remaining_tries: max_tries,
        }
    }

    fn solve(&mut self) -> Option<Branch> {
        let mut i = 0;

        while self.solution.is_none() && self.remaining_tries > 0 {
            i += 1;
            println!("Step: {}, Branches: {}", i, self.branches.len());
            self.step();
        }

        self.solution.clone()
    }

    fn mark_solution(&mut self, branch: Branch) {
        self.solution = Some(branch);
        self.remaining_tries = 0;

        println!("Solution found!");
    }

    fn step(&mut self) {
        if self.remaining_tries == 0 {
            println!("No solution found.");
            return;
        }

        self.remaining_tries = self.remaining_tries.saturating_sub(1);
        let branches = self.branches.clone();
        let mut new_branches = vec![];

        let mut all_history_boards: HashSet<Board> = self.branches.iter()
            .flat_map(|branch| branch.history.iter().map(|entry| entry.board.clone()))
            .chain(self.branches.iter().map(|branch| branch.board.clone()))
            .collect();

        for branch in branches {
            let board_before_steps = branch.board.clone();

            for step in branch.board.get_possible_steps() {
                if all_history_boards.contains(&step.board) {
                    continue;
                }

                all_history_boards.insert(step.board.clone());

                let mut new_history = branch.history.clone();

                new_history.push(HistoryEntry {
                    board: board_before_steps.clone(),
                    pawn_at_move_to: Some((
                        step.pawn_at.0 as u8,
                        step.pawn_at.1 as u8,
                        step.direction as u8,
                    )),
                });

                let mut new_branch = Branch {
                    history: new_history,
                    board: step.board.clone(),
                };

                if step.board.is_solved() {
                    new_branch.history.push(HistoryEntry {
                        board: step.board.clone(),
                        pawn_at_move_to: None,
                    });
                    self.mark_solution(new_branch);
                    return;
                }

                new_branches.push(new_branch);
            }
        }

        self.branches = new_branches;
    }
}

pub fn start_solver(board: Board) -> Option<Solution> {
    let mut board = board;
    board.fulfill_board_with_walls();
    let width = board.width;
    let height = board.height;
    let mut solver = Solver::new(board, 32);

    let the_branch = match solver.solve() {
        Some(branch) => branch,
        None => return None,
    };

    let mut steps = vec![];

    for step in the_branch.history {
        steps.push(step.clone());
    }

    Some(Solution {
        steps,
        width,
        height,
    })
}