pub mod api_structs {
    #[allow(non_snake_case)]

    #[derive(serde::Deserialize)]
    pub struct MoveRequest {
        pub coordinates: [usize; 2],
        pub moveType: MoveType,
    }

    #[allow(non_snake_case)]
    #[derive(serde::Deserialize)]
    pub struct MoveRequestGroup {
        pub moveRequests: Vec<MoveRequest>
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
