use anyhow::anyhow;
use serde::Serialize;

pub type Board = Vec<Vec<SquareState>>;
pub type Vec2 = (usize, usize);

#[derive(Clone, Copy, Serialize)]
pub enum SquareState {
    Alive,
    Dead,
    Empty
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
        }
    }
    pub fn alive_square(&mut self, coords: Vec2, player_id: usize) -> anyhow::Result<()>{
        let curr_player = &mut self.players[player_id];
        let curr_square = &mut curr_player.private_board[coords.0][coords.1];

        if let SquareState::Empty = curr_square {
            *curr_square = SquareState::Alive;
            return Ok(());
        }
        return Err(anyhow!("Invalid Move"));
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
}
