export module Game {
    export enum SquareState {
        Alive = "Alive",
        Dead = "Dead",
        Empty = "Empty",
        Miss = "Miss",
    }

    export enum MoveType {
        AliveSquare = "AliveSquare",
        KillSquare = "KillSquare",
    }
 
    export type Board = SquareState[][];
}
