import { Board, SquareState } from "./game";
import { getRequest } from "./req";

const CANVAS_SIZE = {
    x: 700,
    y: 700,
}
const BOARD_SIZE = {
    x: 10,
    y: 10,
}

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
    return getRequest("/api/updateBoard/{0}");
}

function renderBoard(board: string, board_ctx: CanvasRenderingContext2D) {
    let parsed_board: Board | undefined = JSON.parse(board);

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

    for (let x = 0; x < CANVAS_SIZE.x; x += squaresize.x) {
        for (let y = 0; y < CANVAS_SIZE.y; y += squaresize.y) {
            if (parsed_board[x][y] === SquareState.Alive) {
                board_ctx.fillStyle = "blue";
                board_ctx.fillRect(x, y, x + squaresize.x, y + squaresize.y);
            }
            else if (parsed_board[x][y] === SquareState.Dead) {
                board_ctx.fillStyle = "red";
                board_ctx.fillRect(x, y, x + squaresize.x, y + squaresize.y);
            }
        }
    }
}

main();


