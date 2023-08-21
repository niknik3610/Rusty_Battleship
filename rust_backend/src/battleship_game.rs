pub type Board = Vec<Vec<bool>>;

pub struct Game {
    pub player_1: Player,
    pub player_2: Player,
    pub current_turn: u32,
}
impl Game {
    pub fn new() -> Self{
        const template_array: [[bool; 10]; 10] = [[false; 10]; 10];
        let p1 = Player {
            id: 1,
            private_board: template_array.map(|item| item.to_vec()).to_vec(),
            attack_board: template_array.map(|item| item.to_vec()).to_vec(),
        };

        let p2 = Player {
            id: 2,
            private_board: template_array.map(|item| item.to_vec()).to_vec(),
            attack_board: template_array.map(|item| item.to_vec()).to_vec(),
        };

        return Game {
            player_1: p1,
            player_2: p2,
            current_turn: 0,
        }
    }
}

pub struct Player {
    pub id: u32,
    pub private_board: Board,
    pub attack_board: Board
}
