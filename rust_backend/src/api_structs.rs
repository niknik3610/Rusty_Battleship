pub mod ApiStructs {
    #[allow(non_snake_case)]
    #[derive(serde::Deserialize)]
    pub struct SendMove {
        pub coordinates: [usize; 2],
        pub moveType: MoveType,
    }
    #[derive(serde::Deserialize)]
    pub enum MoveType {
        KillSquare,
        AliveSquare
    }
}
