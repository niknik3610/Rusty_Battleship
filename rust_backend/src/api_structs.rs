pub mod ApiStructs {
    #[derive(serde::Deserialize)]
    pub struct SendMove {
        pub coordinates: [u32; 2],
        pub movetype: MoveType,
    }
    #[derive(serde::Deserialize)]
    pub enum MoveType {
        KillSquare,
        AliveSquare
    }
}
