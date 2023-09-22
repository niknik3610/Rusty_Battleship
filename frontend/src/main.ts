import { Game } from "./game";
import { ApiRequestType } from "./api_request_types";
import { Request } from "./api_request";
import { BoardHandler } from "./board_handler";

export const CANVAS_SIZE = {
    x: 700,
    y: 700,
};

export let CLIENT_ID: number | undefined;

await main();
async function main() {
    const board = document.getElementById("gameboard")! as HTMLCanvasElement;
    const board_ctx = board.getContext("2d")!;
    board.height = CANVAS_SIZE.y;
    board.width = CANVAS_SIZE.x;
    board_ctx.fillStyle = "black";

    // CLIENT_ID = await fetchClientID();  TODO: CHANGE LATER
    CLIENT_ID = 0;

    const id_field = document.getElementById(
        "idStatus",
    )! as HTMLParagraphElement;

    while (CLIENT_ID === undefined) {
        CLIENT_ID = await fetchClientID();

        id_field.textContent = "Failed to Connect to Server, Retrying...";
        id_field.style.color = "red";
    }

    id_field.style.color = "black";
    id_field.textContent = "Your Client ID: " + CLIENT_ID;

    let moveRequestGroup = new ApiRequestType.MoveRequestGroup();

    const sendMoves = document.getElementById(
        "sendMovesButton",
    )! as HTMLButtonElement;
    sendMoves.style.visibility = "visible";

    sendMoves.addEventListener("click", () =>
        CLIENT_ID !== undefined ? moveRequestGroup.resolve(CLIENT_ID!) : {},
    );

    board.addEventListener("click", (event) => {
        const normalizedCoords = BoardHandler.getPlayerClickLocationOnBoard(
            event,
            board.getBoundingClientRect(),
        );

        let squaresize = {
            x: CANVAS_SIZE.x / BoardHandler.BOARD_SIZE.x,
            y: CANVAS_SIZE.y / BoardHandler.BOARD_SIZE.y,
        };

        board_ctx.fillRect(
            normalizedCoords[0] * squaresize.x,
            normalizedCoords[1] * squaresize.y,
            squaresize.x,
            squaresize.y,
        );
    });

    board_ctx.fillRect(0, 0, CANVAS_SIZE.x, CANVAS_SIZE.y);

    moveRequestGroup.push([0, 0], Game.MoveType.AliveSquare);
    moveRequestGroup.push([1, 1], Game.MoveType.AliveSquare);

    setInterval(async () => {
        let board = await BoardHandler.getRefreshedBoard();
        BoardHandler.renderBoard(board, board_ctx);
    }, 500);
}

type ClientIDApiResponse = {
    c_id: number;
};
async function fetchClientID(): Promise<number | undefined> {
    try {
        let result = await Request.postRequest("/api/requestClientID", "");
        let parsedResult: ClientIDApiResponse = JSON.parse(result);
        return parsedResult.c_id;
    } catch (e) {
        console.log(e);
        return undefined;
    }
}
