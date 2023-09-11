import { Game } from "./game";
import { ApiRequestType } from "./api_request_types";
import { Request } from "./api_request";

const CANVAS_SIZE = {
    x: 700,
    y: 700,
}
const BOARD_SIZE = {
    x: 10,
    y: 10,
}
export let CLIENT_ID: number | undefined; 

await main();
async function main() {
    const board = document.getElementById("gameboard")! as HTMLCanvasElement;
    const board_ctx = board.getContext("2d")!;

    board.width = CANVAS_SIZE.x;
    board.height = CANVAS_SIZE.y;
    board_ctx.fillStyle = "black";

    CLIENT_ID = await fetchClientID(); 
    const id_field = document.getElementById("idStatus")! as HTMLParagraphElement;

    while (CLIENT_ID === undefined) {
        CLIENT_ID = await fetchClientID();
        
        id_field.textContent = "Failed to Connect to Server, Retrying...";
        id_field.style.color = "red";
    }

    id_field.style.color = "black";
    id_field.textContent = "Your Client ID: " + CLIENT_ID;

    let moveRequestGroup = new ApiRequestType.MoveRequestGroup(); 

    const sendMoves = document.getElementById("sendMovesButton")! as HTMLButtonElement;
    sendMoves.style.visibility = "visible";
    sendMoves.addEventListener("click", () => 
                               CLIENT_ID !== undefined ? moveRequestGroup.resolve(CLIENT_ID!) : {});

    board_ctx.fillRect(0, 0, CANVAS_SIZE.x, CANVAS_SIZE.y);

    moveRequestGroup.push([0, 0], Game.MoveType.AliveSquare);

    setInterval(async () => {
        let board = await getRefreshedBoard();
        renderBoard(board, board_ctx);
    }, 500);
}

async function getRefreshedBoard(): Promise<string> {
    console.log("fetching board");
    return Request.getRequest("/api/updateBoard/{0}");
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

type ClientIDApiResponse = {
    c_id: number
}
async function fetchClientID(): Promise<number | undefined> {
    try {
        let result = await Request.postRequest("/api/requestClientID", "");
        let parsedResult: ClientIDApiResponse = JSON.parse(result);
        return parsedResult.c_id;
    }
    catch (e) {
        console.log(e);
        return undefined;
    }
}
