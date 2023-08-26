import { getRequest } from "./req";

const BOARD_SIZE = {
    x: 700,
    y: 700
}

function main() {
    const board = document.getElementById("gameboard")! as HTMLCanvasElement;
    const board_ctx = board.getContext("2d")!;

    board.width = BOARD_SIZE.x;
    board.height = BOARD_SIZE.y;

    board_ctx.fillStyle = "red";
    board_ctx.fillRect(0, 0, BOARD_SIZE.x, BOARD_SIZE.y);

    setInterval(refreshBoard, 1000);
}

function refreshBoard(board_ctx: CanvasRenderingContext2D) {
     getRequest("/api/updateBoard").then((res) => {
        console.log(res);
    }); 
}

main();
