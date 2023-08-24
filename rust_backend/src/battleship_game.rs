use anyhow::anyhow;

pub type Board = Vec<Vec<SquareState>>;
pub type Vec2 = (usize, usize);

#[derive(Clone, Copy)]
enum SquareState {
    Alive,
    Dead,
    Empty
}

pub struct Player {
    pub id: u32,
    pub private_board: Board,
    pub attack_board: Board
} 

pub struct Game {
    pub players: Vec<Player>,
    pub current_turn: u32,
} impl Game {
    pub fn new() -> Self{
        const template_array: [[SquareState; 10]; 10] = [[SquareState::Empty; 10]; 10];
        let p1 = Player {
            id: 0,
            private_board: template_array.map(|item| item.to_vec())
                .to_vec(),
            attack_board: template_array.map(|item| item.to_vec())
                .to_vec(),
        };

        let p2 = Player {
            id: 1,
            private_board: template_array.map(|item| item.to_vec())
                .to_vec(),
            attack_board: template_array.map(|item| item.to_vec())
                .to_vec(),
        };

        let players = vec![p1, p2];
        return Game {
            players,
            current_turn: 0,
        }
    }
    pub fn alive_square(&mut self, coords: Vec2, player_id: usize) -> anyhow::Result<()>{
        let curr_player = self.players[player_id];
        let mut curr_square = &curr_player.private_board[coords.0][coords.1];

        if let SquareState::Empty = curr_square {
            *curr_square = SquareState::Alive;
            return Ok(());
        }
        return Err(anyhow!("Invalid Move"));
    }
}
