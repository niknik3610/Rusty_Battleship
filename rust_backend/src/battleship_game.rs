use anyhow::anyhow;
use serde::Serialize;

use crate::api_structs::ApiStructs;

pub type Board = Vec<Vec<SquareState>>;
pub type Vec2 = (usize, usize);

#[derive(Clone, Copy, Serialize)]
pub enum SquareState {
    Alive,
    Dead,
    Empty,
    Miss
}
pub enum GameState {
    Initilization,
    Playing,
    Finished
}

pub struct Player {
    pub id: u32,
    pub private_board: Board,
    pub attack_board: Board,
} 

pub struct Game {
    pub players: Vec<Player>,
    pub current_turn: usize,
    pub connected_players: usize,
    pub game_state: GameState,
} impl Game {
    pub fn new() -> Self{
        const TEMPLATE_BOARD: [[SquareState; 10]; 10] = [[SquareState::Empty; 10]; 10];
        let p1 = Player {
            id: 0,
            private_board: TEMPLATE_BOARD.map(|item| item.to_vec())
                .to_vec(),
            attack_board: TEMPLATE_BOARD.map(|item| item.to_vec())
                .to_vec(),
        };

        let p2 = Player {
            id: 1,
            private_board: TEMPLATE_BOARD.map(|item| item.to_vec())
                .to_vec(),
            attack_board: TEMPLATE_BOARD.map(|item| item.to_vec())
                .to_vec(),
        };

        let players = vec![p1, p2];
        return Game {
            players,
            current_turn: 0,
            connected_players: 0,
            game_state: GameState::Initilization,
        }
    }
    pub fn alive_square(&mut self, coords: Vec2, player_id: usize) -> anyhow::Result<()>{
        if let GameState::Initilization = self.game_state {
            let curr_player = &mut self.players[player_id];
            let curr_square = &mut curr_player.private_board[coords.0][coords.1];

            if let SquareState::Empty = curr_square {
                *curr_square = SquareState::Alive;
                return Ok(());
            }
            return Err(anyhow!("Invalid Move"));
        }
        return Err(anyhow!("Invalid Gamestate Request"));
    }
    pub fn kill_square(&mut self, coords: Vec2, player_id: usize) 
    -> anyhow::Result<ApiStructs::HitSuccess> {
        let enemy_player = (player_id + 1) % 2;
        let enemy_player_board = &mut self.players[enemy_player];
        let curr_square = &mut enemy_player_board.private_board[coords.0][coords.1];

        match curr_square {
            SquareState::Alive => {
                *curr_square = SquareState::Dead;
                self.players[player_id].attack_board[coords.0][coords.1] = SquareState::Dead;
                return Ok(ApiStructs::HitSuccess{success: true});
            },
            SquareState::Empty => { 
                self.players[player_id].attack_board[coords.0][coords.1] = SquareState::Miss;
                return Ok(ApiStructs::HitSuccess{success: false});
            }
            _ => {
                return Err(anyhow!("Invalid Move"))
            }
        }
    }
    pub fn get_board_priv(&self, player_id: usize) -> anyhow::Result<&Board> {
        if player_id >= self.players.len() {
            return Err(anyhow!("Player ID out of bounds"));
        }
        return Ok(&self.players[player_id].private_board);
    }
    pub fn get_board_attack(&self, player_id: usize) -> anyhow::Result<&Board> {
        if player_id >= self.players.len() {
            return Err(anyhow!("Player ID out of bounds"));
        }
        return Ok(&self.players[player_id].attack_board);
    }
    pub fn player_connection(&mut self) -> usize {
        if self.connected_players < 2 {
            self.connected_players += 1;
            return self.connected_players - 1;
        }
        return 2; //spectator
    }
    pub fn advance_game_state(&mut self) {
        match self.game_state {
            GameState::Initilization => self.game_state = GameState::Playing,
            GameState::Playing => self.game_state = GameState::Finished,
            GameState::Finished => self.game_state = GameState::Finished,
        }
    }
}
