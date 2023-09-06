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
let CLIENT_ID: number | undefined; 

await main();
async function main() {
    const board = document.getElementById("gameboard")! as HTMLCanvasElement;
    const board_ctx = board.getContext("2d")!;

    board.width = CANVAS_SIZE.x;
    board.height = CANVAS_SIZE.y;

    board_ctx.fillStyle = "black";

    CLIENT_ID = await fetchClientID(); 
    if (!CLIENT_ID) {
        console.error("Unable to get Client_ID, something went wrong when connecting to server");
        return;
    }   
    const id_field = document.getElementById("id")! as HTMLParagraphElement;
    id_field.textContent = "Your Client ID: " + CLIENT_ID;

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
                board_ctx.fillStyle = "gray"; 
                board_ctx.fillRect(board_pos.x, board_pos.y, squaresize.x, squaresize.y);
            }
            else if (parsed_board[x][y] === Game.SquareState.Dead) {
                board_ctx.fillStyle = "red";
                board_ctx.fillRect(board_pos.x, board_pos.y, squaresize.x, squaresize.y);
            }
            else if (parsed_board[x][y] === Game.SquareState.Miss) {
                board_ctx.fillStyle = "yellow";
                board_ctx.fillRect(board_pos.x, board_pos.y, squaresize.x, squaresize.y);
            }
        }
    }
}

type ClientIDResponse = {
    c_id: number
}
async function fetchClientID(): Promise<number | undefined> {
    try {
        let result = await Request.postRequest("/api/requestClientID", "");
        let parsedResult: ClientIDResponse = JSON.parse(result);
        return parsedResult.c_id;
    }
    catch (e) {
        console.log(e);
        return undefined;
    }
}

