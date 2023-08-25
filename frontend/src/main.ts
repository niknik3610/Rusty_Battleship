const BOARD_SIZE = {
    x: 700,
    y: 700
}

function main() {
    console.log("Runnin'");
    const board = document.getElementById("gameboard")! as HTMLCanvasElement;
    const board_ctx = board.getContext("2d")!;

    board.width = BOARD_SIZE.x;
    board.height = BOARD_SIZE.y;

    board_ctx.fillStyle = "red";
    board_ctx.fillRect(0, 0, BOARD_SIZE.x, BOARD_SIZE.y);
}

main();
