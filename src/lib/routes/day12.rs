// src/lib/routes/day12.rs

// dependencies
use crate::startup::AppState;
use axum::{extract::State, response::IntoResponse};
use axum_macros::debug_handler;
use std::fmt;

// Day 12 data structure - the game board, consists of a 5 x 6 grid, where the grid cells
// are an enum type, playable area of the grid is 4x4
#[derive(Debug)]
pub struct Board {
    grid: [[Cell; 6]; 5],
}

// methods for the Board type
impl Board {
    pub fn new() -> Self {
        let mut grid = [[Cell::Empty; 6]; 5];

        for (row_index, row) in grid.iter_mut().enumerate() {
            for col in 0..6 {
                if row_index == 4 || col == 0 || col == 5 {
                    row[col] = Cell::Wall; // Modify the row directly
                }
            }
        }

        Self { grid }
    }
}

// implement the Default trait for the Board type
impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

// implement the Display trait for the Board type
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Day 12 data structure - an individual cell in the game, grid cells can have 4 variants
#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Wall,
    Cookie,
    Milk,
}

// implement the Display trait for the Cell type
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "⬛"),
            Cell::Wall => write!(f, "⬜"),
            Cell::Cookie => write!(f, "🍪"),
            Cell::Milk => write!(f, "🥛"),
        }
    }
}

// Day 12 Task 1 Handler - current board state, gets the current state of the board
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 1 Handler - Board State", skip(state))]
pub async fn day12_get_board_state(State(state): State<AppState>) -> impl IntoResponse {
    let board = state.game_board.read().await;
    board.to_string()
}

// Day 12 Task 1 Handler - reset the board to an empty state and return it
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 1 Handler - Reset Board", skip(state))]
pub async fn day12_post_reset_board(State(state): State<AppState>) -> impl IntoResponse {
    let mut board = state.game_board.write().await;
    *board = Board::default();
    board.to_string()
}
