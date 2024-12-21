// src/lib/routes/day12.rs

// dependencies
use crate::startup::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_macros::debug_handler;
use std::fmt::{Display, Write};
use std::str::FromStr;

// struct type to represent the grid of tiles
#[derive(Debug)]
pub struct Game {
    pub board: [[Tile; 4]; 4],
    pub status: Option<Outcome>,
}

// enum type to represent the outcome of a game
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Outcome {
    Cookie,
    Milk,
    Neither,
}

// implement the Display trait for the Outcome type
impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let o = match self {
            Self::Cookie => "🍪 wins!",
            Self::Milk => "🥛 wins!",
            Self::Neither => "No winner.",
        };
        f.write_str(o)
    }
}

// methods for the Board type
impl Game {
    // create a new game board
    pub fn new() -> Self {
        let board = [[Tile::Empty; 4]; 4];
        let status = None;

        Self { board, status }
    }

    // get the contents of any arbitrary tile in the game board, given it's x and y coordinates
    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.board[x][y]
    }

    // test if there is a game winner
    pub fn check_winner(&self, x: usize, y: usize) -> bool {
        // the current tile coordinates
        let tile = self.get_tile(x, y);

        // define the possible winning combinations as an array of tuples
        let top_to_bottom = [(x, 0), (x, 1), (x, 2), (x, 3)];
        let left_to_right = [(0, y), (1, y), (2, y), (3, y)];
        let left_diagonal = [(0, 3), (1, 2), (2, 1), (3, 0)];
        let right_diagonal = [(0, 0), (1, 1), (2, 2), (3, 3)];

        // define a closure which, when given the current position, check the board against the above defined winning combinations
        let t_check = |pos: &[(usize, usize)]| -> bool {
            pos.contains(&(x, y))
                && pos
                    .iter()
                    .map(|(x, y)| self.get_tile(*x, *y))
                    .all(|t| t == tile)
        };

        // call the closure with each of the winning combinations to check if any are true
        t_check(&top_to_bottom)
            || t_check(&left_to_right)
            || t_check(&left_diagonal)
            || t_check(&right_diagonal)
    }

    // make a move by placing an item
    pub fn make_move(&mut self, tile: Tile, column: usize) -> bool {
        if self.status.is_some() {
            return false;
        }

        let (y, find_tile) = match self.board[column]
            .iter_mut()
            .enumerate()
            .find(|(_, t)| t == &&Tile::Empty)
        {
            Some(n) => n,
            None => return false,
        };
        *find_tile = tile;

        let is_full = self
            .board
            .iter()
            .flatten()
            .find(|&&t| t == Tile::Empty)
            .is_none();

        if is_full {
            self.status = Some(Outcome::Neither);
        }

        if self.check_winner(column, y) {
            self.status = Some(match tile {
                Tile::Cookie => Outcome::Cookie,
                Tile::Milk => Outcome::Milk,
                Tile::Empty => unreachable!(),
            });
        }

        true
    }
}

// implement the Default trait for the Game type
impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

// implement the Display trait for the Game type
impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wall = '⬜';
        for y in (0..4).rev() {
            f.write_char(wall)?;
            for x in 0..4 {
                f.write_fmt(format_args!("{}", self.get_tile(x, y)))?
            }
            f.write_fmt(format_args!("{}\n", wall))?
        }
        f.write_fmt(format_args!(
            "{}{}{}{}{}{}\n",
            wall, wall, wall, wall, wall, wall
        ))?;

        if let Some(outcome) = self.status {
            f.write_fmt(format_args!("{}\n", outcome))?;
        }

        Ok(())
    }
}

// enum type to represent the contents of a tile
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Cookie,
    Milk,
}

// implement the Display trait for the Tile type
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            Self::Empty => '⬛',
            Self::Cookie => '🍪',
            Self::Milk => '🥛',
        };
        f.write_char(t)
    }
}

// implement the FromStr trait for the Team type, used to convert the path team path paramter into
// the Team enum type
impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cookie" => Ok(Tile::Cookie),
            "milk" => Ok(Tile::Milk),
            _ => Err(()),
        }
    }
}

// Day 12 Task 1 Handler - current board state, gets the current state of the game board
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 1 Handler - Board State", skip(state))]
pub async fn day_12_get_board_state(State(state): State<AppState>) -> impl IntoResponse {
    let game = state.game.read().await;
    let response_body = format!("{}", game);
    (StatusCode::OK, response_body).into_response()
}

// Day 12 Task 1 Handler - reset the board to an empty state and return it
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 1 Handler - Reset Board", skip(state))]
pub async fn day12_post_reset_board(State(state): State<AppState>) -> impl IntoResponse {
    let mut game = state.game.write().await;
    *game = Game::default();
    let response_body = format!("{}", game);
    (StatusCode::OK, response_body).into_response()
}

// Day 12, Task 2 Handler - place an item into the board
#[debug_handler]
#[tracing::instrument(name = "Day 12 Task 2 Handler - Place an Item", skip(state))]
pub async fn day12_post_place_item(
    State(state): State<AppState>,
    Path(path): Path<(String, String)>,
) -> impl IntoResponse {
    let tile = match Tile::from_str(&path.0) {
        Ok(tile) => tile,
        Err(_) => return (StatusCode::BAD_REQUEST).into_response(),
    };

    let column: usize = match path.1.parse::<usize>() {
        Ok(num) if (1..=4).contains(&num) => num,
        _ => return (StatusCode::BAD_REQUEST).into_response(),
    };

    let mut game = state.game.write().await;

    if game.make_move(tile, column - 1) {
        let response_body = format!("{}", game);
        return (StatusCode::OK, response_body).into_response();
    } else {
        let response_body = format!("{}", game);
        return (StatusCode::SERVICE_UNAVAILABLE, response_body).into_response();
    }
}

// unit tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default_method_returns_a_new_game_with_an_empty_board() {
        let test_game = Game::default();
        assert_eq!(test_game.board, [[Tile::Empty; 4]; 4]);
        assert_eq!(test_game.status, None);
    }

    #[test]
    fn get_tile_returns_the_tile_contents() {
        let mut test_game = Game::default();
        test_game.board[1][2] = Tile::Cookie;
        let tile = test_game.get_tile(1, 2);

        assert_eq!(tile, Tile::Cookie);
        assert_eq!(test_game.get_tile(0, 0), Tile::Empty)
    }

    #[test]
    fn check_winner_returns_true_for_winning_conditions() {
        // top to bottom winning pattern
        let test_game = Game {
            board: [
                [Tile::Cookie, Tile::Milk, Tile::Milk, Tile::Cookie],
                [Tile::Cookie, Tile::Cookie, Tile::Cookie, Tile::Cookie],
                [Tile::Milk, Tile::Milk, Tile::Milk, Tile::Cookie],
                [Tile::Milk, Tile::Milk, Tile::Cookie, Tile::Cookie],
            ],
            status: None,
        };

        assert!(test_game.check_winner(1, 1));

        // left_to_right winning pattern
        let test_game = Game {
            board: [
                [Tile::Cookie, Tile::Milk, Tile::Milk, Tile::Milk],
                [Tile::Cookie, Tile::Cookie, Tile::Cookie, Tile::Milk],
                [Tile::Milk, Tile::Milk, Tile::Milk, Tile::Milk],
                [Tile::Milk, Tile::Milk, Tile::Cookie, Tile::Cookie],
            ],
            status: None,
        };

        assert!(test_game.check_winner(2, 0));

        // left to right diagonal winning pattern
        let test_game = Game {
            board: [
                [Tile::Cookie, Tile::Milk, Tile::Milk, Tile::Milk],
                [Tile::Cookie, Tile::Cookie, Tile::Cookie, Tile::Milk],
                [Tile::Milk, Tile::Milk, Tile::Cookie, Tile::Milk],
                [Tile::Milk, Tile::Milk, Tile::Cookie, Tile::Cookie],
            ],
            status: None,
        };

        assert!(test_game.check_winner(0, 0));

        // right to left diagonal winning pattern
        let test_game = Game {
            board: [
                [Tile::Cookie, Tile::Milk, Tile::Milk, Tile::Milk],
                [Tile::Cookie, Tile::Cookie, Tile::Milk, Tile::Milk],
                [Tile::Milk, Tile::Milk, Tile::Milk, Tile::Milk],
                [Tile::Milk, Tile::Milk, Tile::Cookie, Tile::Cookie],
            ],
            status: None,
        };

        assert!(test_game.check_winner(3, 0));
    }
}
