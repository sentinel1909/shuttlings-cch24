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

    pub fn make_move(&mut self, team: Team, column: usize) -> Result<(), &'static str> {
        let column_idx = column - 1;

        for row in (0..4).rev() {
            if let Cell::Empty = self.grid[row][column_idx + 1] {
                self.grid[row][column_idx + 1] = match team {
                    Team::Milk => Cell::Milk,
                    Team::Cookie => Cell::Cookie,
                };
                return Ok(());
            }
        }

        Err("Column is full.")
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
#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Status {
    Ongoing,
    Winner(Team),
    Draw,
}

// methods for the Game type
impl Game {
    pub fn new() -> Self {
        let board = Board::default();
        let status = Status::Ongoing;

        Self { board, status }
    }
}

// implement the Default trait for the Game type
impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

// Day 12 data structure - an enum type to represent the player teams
#[derive(Clone, Debug, Deserialize, PartialEq)]
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
            "cookie" => Ok(Team::Cookie),
            "milk" => Ok(Team::Milk),
            _ => Err(()),
        }
    }
}

// helper function to determine the game status
fn check_game_progression(board: &Board) -> Status {
    let rows = 4;
    let cols = 4;

    fn check_line(
        grid: &[[Cell; 6]; 5],
        start_row: usize,
        start_col: usize,
        delta_row: isize,
        delta_col: isize,
        team_cell: Cell,
    ) -> bool {
        for i in 0..4 {
            let row = start_row as isize + i * delta_row;
            let col = start_col as isize + i * delta_col;

            if row < 0
                || !(0..=4).contains(&row)
                || !(1..=4).contains(&col)
                || grid[row as usize][col as usize] != team_cell
            {
                return false;
            }
        }
        true
    }

    for row in 0..rows {
        for col in 1..=cols {
            if let Cell::Empty = board.grid[row][col] {
                continue;
            }

            let current_cell = board.grid[row][col];

            if (current_cell == Cell::Cookie || current_cell == Cell::Milk)
                && (check_line(&board.grid, row, col, 0, 1, current_cell)
                    || check_line(&board.grid, row, col, 1, 0, current_cell)
                    || check_line(&board.grid, row, col, 1, 1, current_cell)
                    || check_line(&board.grid, row, col, 1, -1, current_cell))
            {
                return Status::Winner(match current_cell {
                    Cell::Milk => Team::Milk,
                    Cell::Cookie => Team::Cookie,
                    _ => unreachable!(),
                });
            }
        }
    }

    let is_full = board
        .grid
        .iter()
        .flat_map(|row| row.iter())
        .all(|&cell| cell != Cell::Empty);

    if is_full {
        Status::Draw
    } else {
        Status::Ongoing
    }
}

// Day 12 Task 1 Handler - current board state, gets the current state of the board
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 1 Handler - Board State", skip(state))]
pub async fn day12_get_board_state(State(state): State<AppState>) -> impl IntoResponse {
    let game = state.game.read().await;
    let status_message = match &game.status {
        Status::Winner(winning_team) => format!(
            "{} wins!",
            if *winning_team == Team::Cookie {
                "🍪"
            } else {
                "🥛"
            }
        ),
        Status::Draw => "No winner.".to_string(),
        Status::Ongoing => "".to_string(), // No message for ongoing games
    };
    let response_body = if status_message.is_empty() {
        format!("{}", game.board)
    } else {
        format!("{}{}", game.board, status_message)
    };
    (StatusCode::OK, response_body).into_response()
}

// Day 12 Task 1 Handler - reset the board to an empty state and return it
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 1 Handler - Reset Board", skip(state))]
pub async fn day12_post_reset_board(State(state): State<AppState>) -> impl IntoResponse {
    let mut game = state.game.write().await;
    *game = Game::default();
    let response_body = format!("{}", game.board);
    response_body
}

// Day 12 Task 2 Handler - enables teams to make a move
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 2 Handler - Make a Move", skip(state))]
pub async fn day12_post_play_game(
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

    let mut game = state.game.write().await;

    match &game.status {
        Status::Winner(winning_team) => {
            let response_body = format!(
                "{}\n{} wins!",
                game.board,
                if *winning_team == Team::Cookie {
                    "🍪"
                } else {
                    "🥛"
                }
            );
            return (StatusCode::SERVICE_UNAVAILABLE, response_body).into_response();
        }
        Status::Draw => {
            let response_body = format!("{}No winner.", game.board);
            return (StatusCode::SERVICE_UNAVAILABLE, response_body).into_response();
        }
        Status::Ongoing => {}
    }

    if game.board.make_move(team, column).is_err() {
        let response_body = format!("{}", game.board);
        return (StatusCode::SERVICE_UNAVAILABLE, response_body).into_response();
    }

    match check_game_progression(&game.board) {
        Status::Winner(winning_team) => {
            game.status = Status::Winner(winning_team.clone());
            let response_body = format!(
                "{}\n{}",
                game.board,
                if winning_team == Team::Cookie {
                    "🍪 wins!"
                } else {
                    "🥛 wins!"
                }
            );
            return (StatusCode::SERVICE_UNAVAILABLE, response_body).into_response();
        }
        Status::Draw => {
            game.status = Status::Draw;
            let response_body = format!("{}\nNo winner.", game.board);
            return (StatusCode::OK, response_body).into_response();
        }
        Status::Ongoing => {}
    }

    let response_body = format!("{}", game.board);
    (StatusCode::OK, response_body).into_response()
}
