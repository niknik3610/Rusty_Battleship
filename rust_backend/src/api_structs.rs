pub mod ApiStructs {
    #[allow(non_snake_case)]

    #[derive(serde::Deserialize)]
    pub struct Move {
        pub coordinates: [usize; 2],
        pub moveType: MoveType,
    }

    #[derive(serde::Deserialize)]
    pub struct SendMove {
        pub moves: Vec<Move>
    }

    #[derive(serde::Deserialize)]
    pub enum MoveType {
        KillSquare,
        AliveSquare
    }

    #[derive(serde::Serialize)]
    pub struct HitSuccess {
        pub success: bool,
    }

    #[derive(serde::Serialize)]
    pub struct ClientID {
        pub c_id: usize,
    }
}
