import { Game } from "./game";
import { Request } from "./api_request";
import { CANVAS_SIZE } from "./main";

export module BoardHandler {
    export const BOARD_SIZE = {
        x: 10,
        y: 10,
    };

    export async function getRefreshedBoard(): Promise<string> {
        console.log("fetching board");
        return Request.getRequest("/api/updateBoard/{0}");
    }

    export function renderBoard(
        board: string,
        board_ctx: CanvasRenderingContext2D,
    ) {
        let parsed_board: Game.Board | undefined = JSON.parse(board);

        if (!parsed_board) {
            console.log("Request failed to parse");
            return;
        }

        console.log(parsed_board);

        let squaresize = {
            x: CANVAS_SIZE.x / BOARD_SIZE.x,
            y: CANVAS_SIZE.y / BOARD_SIZE.y,
        };

        for (let x = 0; x < BOARD_SIZE.x; x += 1) {
            for (let y = 0; y < BOARD_SIZE.y; y += 1) {
                let board_pos = {
                    x: x * squaresize.x,
                    y: y * squaresize.y,
                };

                if (parsed_board[x][y] === Game.SquareState.Alive) {
                    board_ctx.fillStyle = "gray";
                    board_ctx.fillRect(
                        board_pos.x,
                        board_pos.y,
                        squaresize.x,
                        squaresize.y,
                    );
                } else if (parsed_board[x][y] === Game.SquareState.Dead) {
                    board_ctx.fillStyle = "red";
                    board_ctx.fillRect(
                        board_pos.x,
                        board_pos.y,
                        squaresize.x,
                        squaresize.y,
                    );
                } else if (parsed_board[x][y] === Game.SquareState.Miss) {
                    board_ctx.fillStyle = "yellow";
                    board_ctx.fillRect(
                        board_pos.x,
                        board_pos.y,
                        squaresize.x,
                        squaresize.y,
                    );
                }
            }
        }
    }

    export function getPlayerClickLocationOnBoard(
        event: MouseEvent,
        canvasRect: DOMRect,
    ) {
        const normalizedCoords = [
            event.clientX - canvasRect.x,
            event.clientY - canvasRect.y,
        ];

        const locationsOnBoard = [
            Math.floor((normalizedCoords[0] / CANVAS_SIZE.x) * BOARD_SIZE.x),
            Math.floor((normalizedCoords[1] / CANVAS_SIZE.y) * BOARD_SIZE.y),
        ];

        console.log(locationsOnBoard[0] + ", ", locationsOnBoard[1]);

        return locationsOnBoard;
    }
}
