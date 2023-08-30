import { Game } from "./game";
import { Request } from "./req";

const CANVAS_SIZE = {
    x: 700,
    y: 700,
}
const BOARD_SIZE = {
    x: 10,
    y: 10,
}
const CLIENT_ID = 0;

function main() {
    const board = document.getElementById("gameboard")! as HTMLCanvasElement;
    const board_ctx = board.getContext("2d")!;

    board.width = CANVAS_SIZE.x;
    board.height = CANVAS_SIZE.y;

    board_ctx.fillStyle = "black";
    board_ctx.fillRect(0, 0, CANVAS_SIZE.x, CANVAS_SIZE.y);

    setInterval(async () => {
        let board = await getRefreshedBoard();
        renderBoard(board, board_ctx);
    }, 1000);
}

async function getRefreshedBoard(): Promise<string> {
    console.log("fetching board");
    return Request.getRequest("/api/updateBoard/{0}");
}

async function sendMove(coordinates: [number, number], moveType: Game.MoveType) {
    let req_content = JSON.stringify({
        coordiantes: coordinates,
        movetype: moveType,
    });

    Request.postRequest(`api/sendMove/{${CLIENT_ID}}`, req_content);
}

function renderBoard(board: string, board_ctx: CanvasRenderingContext2D) {
    let parsed_board: Game.Board | undefined = JSON.parse(board);

    if (!parsed_board) {
        console.log("Request failed to parse");
        return;
    }

    console.log(parsed_board);

    board_ctx.fillStyle = "blue";
    let squaresize = {
        x: CANVAS_SIZE.x/BOARD_SIZE.x,
        y: CANVAS_SIZE.y/BOARD_SIZE.y
    }
    
    for (let x = 0; x < BOARD_SIZE.x; x += 1) {
        for (let y = 0; y < BOARD_SIZE.y; y+= 1) {
            let board_pos = {
                x: x * squaresize.x,
                y: y * squaresize.y,
            }
            
            if (parsed_board[x][y] === Game.SquareState.Alive) {
                board_ctx.fillStyle = "blue"; 
                board_ctx.fillRect(board_pos.x, board_pos.y, board_pos.x + squaresize.x, board_pos.y + squaresize.y);

            }
            else if (parsed_board[x][y] === Game.SquareState.Dead) {
                board_ctx.fillStyle = "red";
                board_ctx.fillRect(board_pos.x, board_pos.y, board_pos.x + squaresize.x, board_pos.y + squaresize.y);
            }
        }
    }
}

main();


