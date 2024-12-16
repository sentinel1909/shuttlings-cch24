// src/lib/routes/day12.rs

// dependencies
use crate::startup::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;
use serde::Deserialize;
use std::fmt;
use std::str::FromStr;

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
            for (col_index, cell) in row.iter_mut().enumerate() {
                if row_index == 4 || col_index == 0 || col_index == 5 {
                    *cell = Cell::Wall;
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

// Day 12 data structure - a struct type to represent a game
#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub status: Status,
}

// methods for the Game type
impl Game {
    pub fn new() -> Self {
        let board = Board::default();
        let status = Status::Ongoing;

        Self { board, status }
    }

    pub fn make_move(team: Team, column: usize) -> Self {
        todo!()
    }
}

// implement the Default trait for the Game type
impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

// Day 12 data structure - an enum type to represent game progress
#[derive(Debug)]
pub enum Status {
    Winner,
    Draw,
    Ongoing,
}

// Day 12 data structure - an enum type to represent the player teams
#[derive(Debug, Deserialize)]
pub enum Team {
    Milk,
    Cookie,
}

// implement the FromStr trait for the Team type, used to convert the path team path paramter into
// the Team enum type
impl FromStr for Team {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cooke" => Ok(Team::Cookie),
            "milk" => Ok(Team::Milk),
            _ => Err(()),
        }
    }
}

// helper function to determine the game status
fn check_game_progression(board: &Board) -> Status {
    todo!()
}

// Day 12 Task 1 Handler - current board state, gets the current state of the board
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 1 Handler - Board State", skip(state))]
pub async fn day12_get_board_state(State(state): State<AppState>) -> impl IntoResponse {
    let game = state.game.read().await;
    game.board.to_string()
}

// Day 12 Task 1 Handler - reset the board to an empty state and return it
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 1 Handler - Reset Board", skip(state))]
pub async fn day12_post_reset_board(State(state): State<AppState>) -> impl IntoResponse {
    let mut game = state.game.write().await;
    *game = Game::default();
    game.board.to_string()
}

// Day 12 Task 2 Handler - enables teams to make a move
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 2 Handler - Make a Move", skip(state))]
pub async fn day12_post_make_move(
    State(state): State<AppState>,
    Path(path): Path<(String, String)>,
) -> impl IntoResponse {
    let team = match Team::from_str(&path.0) {
        Ok(team) => team,
        Err(_) => return (StatusCode::BAD_REQUEST).into_response(),
    };

    let column: usize = match path.1.parse::<usize>() {
        Ok(num) if (1..=4).contains(&num) => num,
        _ => return (StatusCode::BAD_REQUEST).into_response(),
    };

    let game = state.game.read().await;

    match check_game_progression(&game.board) {
        Status::Winner => {
            return (StatusCode::SERVICE_UNAVAILABLE).into_response();
        }
        Status::Draw => {
            return (StatusCode::SERVICE_UNAVAILABLE).into_response();
        }
        Status::Ongoing => {
            let mut game = state.game.write().await;
            *game = Game::make_move(team, column);
            return (StatusCode::OK).into_response();
        }
    }
}
