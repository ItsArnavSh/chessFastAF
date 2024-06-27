<script lang="ts">
  import { onMount } from "svelte";
  import blackPawnSrc from "./pieces/pawn-b.svg";
  import blackKingSrc from "./pieces/king-b.svg";
  import blackQueenSrc from "./pieces/queen-b.svg";
  import blackRookSrc from "./pieces/rook-b.svg";
  import blackKnightSrc from "./pieces/knight-b.svg";
  import blackBishopSrc from "./pieces/bishop-b.svg";
  import {get_moves} from "../pkg/chess_fast_af"
  import whitePawnSrc from "./pieces/pawn-w.svg";
  import whiteKingSrc from "./pieces/king-w.svg";
  import whiteQueenSrc from "./pieces/queen-w.svg";
  import whiteRookSrc from "./pieces/rook-w.svg";
  import whiteKnightSrc from "./pieces/knight-w.svg";
  import whiteBishopSrc from "./pieces/bishop-w.svg";
  import { coordToNum, mapToNumber, mapToString, numToCoord, stringToMap, numberToMap } from "./lib/helper";
  import moveDotSrc from "./pieces/blackDot.png";

  // Preload images
  const blackPawn = new Image();
  blackPawn.src = blackPawnSrc;
  const blackKing = new Image();
  blackKing.src = blackKingSrc;
  const blackQueen = new Image();
  blackQueen.src = blackQueenSrc;
  const blackRook = new Image();
  blackRook.src = blackRookSrc;
  const blackKnight = new Image();
  blackKnight.src = blackKnightSrc;
  const blackBishop = new Image();
  blackBishop.src = blackBishopSrc;

  const whitePawn = new Image();
  whitePawn.src = whitePawnSrc;
  const whiteKing = new Image();
  whiteKing.src = whiteKingSrc;
  const whiteQueen = new Image();
  whiteQueen.src = whiteQueenSrc;
  const whiteRook = new Image();
  whiteRook.src = whiteRookSrc;
  const whiteKnight = new Image();
  whiteKnight.src = whiteKnightSrc;
  const whiteBishop = new Image();
  whiteBishop.src = whiteBishopSrc;

  const moveDot = new Image();
  moveDot.src = moveDotSrc;

  let board: bigint[] = [
    0b1111111111111111000000000000000000000000000000000000000000000000n, // All Black Pieces
    0b0000000000000000000000000000000000000000000000001111111111111111n, // All White Pieces
    0b0000100000000000000000000000000000000000000000000000000000000000n, // Black King
    0b0001000000000000000000000000000000000000000000000000000000000000n, // Black Queen
    0b1000000100000000000000000000000000000000000000000000000000000000n, // Black Rook
    0b0100001000000000000000000000000000000000000000000000000000000000n, // Black Knight
    0b0010010000000000000000000000000000000000000000000000000000000000n, // Black Bishop
    0b0000000011111111000000000000000000000000000000000000000000000000n, // Black Pawn
    0b0000000000000000000000000000000000000000000000000000000000001000n, // White King
    0b0000000000000000000000000000000000000000000000000000000000010000n, // White Queen
    0b0000000000000000000000000000000000000000000000000000000010000001n, // White Rook
    0b0000000000000000000000000000000000000000000000000000000001000010n, // White Knight
    0b0000000000000000000000000000000000000000000000000000000000100100n, // White Bishop
    0b0000000000000000000000000000000000000000000000001111111100000000n, // White Pawn
    0b000000000000000011111n, // The Special One 
  ];

  let moves: bigint = 0b0n;
  let canvas: HTMLCanvasElement;
  let dragCanvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;
  let dragCtx: CanvasRenderingContext2D;
  let width: number = window.innerWidth;
  let height: number = window.innerHeight;
  let cursorX: number = 0, cursorY: number = 0;
  let isDragging = false;
  let a: number = 0;
  let cstart = 1;
  let cend = 1;
  let location = 0;
  let helper = 0;
  let Clicked = false;
  let assigner: number[][] = [];
  let clickStart: number;
  let moves2 = 0n
  let dragImg: HTMLImageElement;
  width = window.innerWidth;
  height = window.innerHeight;
  let side = Math.min(width, height) * 0.8;
  let hideNow: number;
  let selected :number;
  function handleMouseMove(event: MouseEvent) {
    const rect = canvas.getBoundingClientRect();
    cursorX = event.clientX - rect.left;
    cursorY = event.clientY - rect.top;
    identifySquare();
    if (isDragging) {
      requestAnimationFrame(() => {
        dragCtx.clearRect(0, 0, dragCanvas.width, dragCanvas.height);
        dragCtx.drawImage(dragImg, cursorX - side / 16, cursorY - side / 16, side / 8, side / 8);
      });
    } else {
      dragCtx.clearRect(0, 0, width, height);
    }
  }

  onMount(() => {
    if (!canvas || !dragCanvas) {
      throw new Error("Canvas is null");
    }
    canvas.width = side;
    canvas.height = side;
    dragCanvas.width = side;
    dragCanvas.height = side;
    ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
    dragCtx = dragCanvas.getContext("2d") as CanvasRenderingContext2D;
    if (!ctx || !dragCtx) {
      throw new Error("Unable to get canvas context");
    }
    makeBoxes();
    directions();
    display();
  });

  function directions() {
    const dim = side / 8;
    for (let i = 0; i < 8; i++) {
      for (let j = 0; j < 8; j++) {
        assigner.push([Math.trunc(dim * j), Math.trunc(dim * i)]);
      }
    }
  }

  function identifySquare() {
    const dim = side / 8;
    let a = Math.trunc(1 + cursorY / dim);
    let b = Math.trunc(1 + cursorX / dim);
    if (a > 0 && a < 9 && b > 0 && b < 9)
      location = 8 * (a - 1) + b - 1;
  }

  function makeBoxes() {
    const dim = side / 8;
    let filler = '#E9EDCC';
    let [xcoord, ycoord] = [0, 0];
    for (let i = 0; i < 8; i++) {
      for (let j = 0; j < 8; j++) {
        if (j != 0) {
          filler = (filler === "#779954") ? '#E9EDCC' : '#779954';
        }
        ctx.fillStyle = filler;
        ctx.fillRect(xcoord, ycoord, dim, dim);
        xcoord += dim;
      }
      xcoord = 0;
      ycoord += dim;
    }
  }

  function dragStart() {
    isDragging = true;
    moves = showMoves(numberToMap(location));
    dragImg = new Image();
    selectedImage(location);
    makeBoxes();
    displayNormal();
    cstart = location;
  }

  function dragEnd() {
    console.log("Drag End")
    if (cstart != location && moves & numberToMap(location)) {
      let start = numberToMap(cstart);
      let end = numberToMap(location);
      
      for (let i = 2; i <= 13; i++) {
        if (start & board[i]) {
          board[i] = start ^ board[i];
          board[i] = end | board[i];
          moves = 0n;
          makeBoxes();
          displayNormal();
          break;
        }
      }
      for(let i= 0;i<2;i++)
      {
        if (start & board[i]) {
          board[i] = start ^ board[i];
          board[i] = end | board[i];
          board[14]=board[14]^0b1n;
          board[Number(board[14]&1n)?1:0]&=~end;
          for(let j = 2;j<=7;j++)
          {
            board[j]&=board[0]
          }
          for(let j = 8;j<=13;j++)
          {
            board[j]&=board[1]
          }
        }
      }
    }
    dragCtx.clearRect(0, 0, dragCanvas.width, dragCanvas.height);
    isDragging = false;
    hideNow = 65;
    displayNormal();
    cend = location;
    moves = 0n;
  }

  function handleClick() {
  console.log("Clicked");

  if (isDragging) {
    // If a drag is in progress, do nothing on click
    return;
  }

  if (cstart === cend) {
    // Single click on the same location
    if (moves2 !== 0n) {
      selected = -1
      // If there are moves available, handle the click as move selection
      if (moves2 & numberToMap(location)) {
        // Check if the clicked location is a valid move
        let start = numberToMap(clickStart);
        let end = numberToMap(location);
        
        for (let i = 2; i <= 13; i++) {
          if (start & board[i]) {
            // Update the board state for the selected piece
            board[i] = start ^ board[i];
            board[i] = end | board[i];
            break;
          }
        }
        for(let i= 0;i<2;i++)
      {
        if (start & board[i]) {
          board[i] = start ^ board[i];
          board[i] = end | board[i];
          board[14]=board[14]^0b1n;
          board[Number(board[14]&1n)?1:0]&=~end;
          for(let j = 2;j<=7;j++)
          {
            board[j]&=board[0]
          }
          for(let j = 8;j<=13;j++)
          {
            board[j]&=board[1]
          }
        }
      }
        moves2 = 0n; // Clear moves after executing the move
      }
    } else {
      // If no moves are available, handle the click as initial piece selection
      clickStart = location;
      cstart = location;
      selected = location
      //selectedImage(location); // Update selected image state if necessary
      moves2 = showMoves(numberToMap(location)); // Show available moves for the selected piece
    }
  }
  else
  {
    selected = -1
    moves2 = 0n
  }

  makeBoxes(); // Redraw the board
  displayNormal(); // Display pieces according to the updated board state
}



  function showMoves(clickMap:bigint):bigint {
    if(validClick(clickMap)!=-1 && BigInt(turnDetect(clickMap))==(board[14]&1n))
    {
        console.log("Valid Click")
        let result:bigint = processSingleBigInt(get_moves(breakToSend(clickMap)))
        return result;
    }
    console.log("Not a valid click")
    return 0n
  }
  //We need to break the bigints into two
  function breakToSend(clickMap:bigint)
  {
    let storage:number[] = []
    for(let i = 0;i<=14;i++)
    {
      storage.push(Number(board[i]>>32n))//Upper 32
      storage.push(Number(board[i]&0b11111111111111111111111111111111n))//Lower 32
    }
    storage.push(Number(clickMap>>32n))
    storage.push(Number(clickMap&0b11111111111111111111111111111111n))
    let arr:Uint32Array = new Uint32Array(storage)
    console.log(arr.length)
    return(arr)
  }
  function processSingleBigInt(bitarray:Uint32Array)
  {
    console.log((BigInt(bitarray[0])<<32n)+BigInt(bitarray[1]))
    return (BigInt(bitarray[0])<<32n)+BigInt(bitarray[1]);
  }

  function display() {
    setTimeout(() => {
      displayType(mapToNumber(board[2])[0], blackKing);
      displayType(mapToNumber(board[8])[0], whiteKing);
    }, 1100);

    setTimeout(() => {
      mapToNumber(board[7]).map((x) => { displayType(x, blackPawn); });
      mapToNumber(board[13]).map((x) => { displayType(x, whitePawn); });
    }, 100);

    setTimeout(() => {
      mapToNumber(board[3]).map((x) => { displayType(x, blackQueen); });
    mapToNumber(board[9]).map((x) => { displayType(x, whiteQueen); });
    }, 900);

    setTimeout(() => {
      mapToNumber(board[4]).map((x) => { displayType(x, blackRook); });
      mapToNumber(board[10]).map((x) => { displayType(x, whiteRook); });
    }, 300);

    setTimeout(() => {
      mapToNumber(board[5]).map((x) => { displayType(x, blackKnight); });
      mapToNumber(board[11]).map((x) => { displayType(x, whiteKnight); });
    }, 500);

    setTimeout(() => {
      mapToNumber(board[6]).map((x) => { displayType(x, blackBishop); });
      mapToNumber(board[12]).map((x) => { displayType(x, whiteBishop); });
    }, 700);
  }

  function displayNormal() {
    displayType(mapToNumber(board[2])[0], blackKing);
    displayType(mapToNumber(board[8])[0], whiteKing);

    mapToNumber(board[7]).map((x) => { displayType(x, blackPawn); });
    mapToNumber(board[13]).map((x) => { displayType(x, whitePawn); });

    mapToNumber(board[3]).map((x) => { displayType(x, blackQueen); });
    mapToNumber(board[9]).map((x) => { displayType(x, whiteQueen); });

    mapToNumber(board[4]).map((x) => { displayType(x, blackRook); });
    mapToNumber(board[10]).map((x) => { displayType(x, whiteRook); });

    mapToNumber(board[5]).map((x) => { displayType(x, blackKnight); });
    mapToNumber(board[11]).map((x) => { displayType(x, whiteKnight); });

    mapToNumber(board[6]).map((x) => { displayType(x, blackBishop); });
    mapToNumber(board[12]).map((x) => { displayType(x, whiteBishop); });

    mapToNumber(moves).map((x) => { displayType(x, moveDot); });
    mapToNumber(moves2).map((x) => { displayType(x, moveDot); });
  }

  function displayType(coord: number, img: HTMLImageElement) {
    if (coord != hideNow) {
      helper = coord;
      if (img === moveDot) {
        let [x, y] = assigner[coord];
        ctx.beginPath();
        ctx.arc(x + side / 16, y + side / 16, side / 30, 0, 2 * Math.PI);
        if(numberToMap(coord)&(board[1]|board[0]))
        ctx.fillStyle = ("#FF0000");
      else
        ctx.fillStyle = ("#000000");
        ctx.fill();
        ctx.beginPath();
        ctx.arc(x + side / 16, y + side / 16, side / 40, 0, 2 * Math.PI);
        ctx.fillStyle = ("#ffffff");
        ctx.fill();
      } else {
        let [x, y] = assigner[coord];
        if(coord == selected)
    {
      ctx.fillStyle = '#FFFF00'
      ctx.fillRect(x,y,side/8,side/8)
    }
        
        ctx.drawImage(img, x, y, side / 8, side / 8);
      }
    }
    
  }

  function selectedImage(coord: number) {
    let clickLocation = numberToMap(coord);
    for (let i = 2; i <= 13; i++) {
      if (clickLocation & board[i]) {
        hideNow = coord;
        switch (i) {
          case 2:
            dragImg = blackKing;
            break;
          case 3:
            dragImg = blackQueen;
            break;
          case 4:
            dragImg = blackRook;
            break;
          case 5:
            dragImg = blackKnight;
            break;
          case 6:
            dragImg = blackBishop;
            break;
          case 7:
            dragImg = blackPawn;
            break;
          case 8:
            dragImg = whiteKing;
            break;
          case 9:
            dragImg = whiteQueen;
            break;
          case 10:
            dragImg = whiteRook;
            break;
          case 11:
            dragImg = whiteKnight;
            break;
          case 12:
            dragImg = whiteBishop;
            break;
          case 13:
            dragImg = whitePawn;
            break;
        }
      }
    }
  }
  //Now lets get started with the actual game
  function validClick(clickMap:bigint):number
  {
    for(let i = 2;i<14;i++)
    {
      if(board[i]&clickMap)
        return i
    }
    return -1
  }
  function turnDetect(clickMap:bigint):number
  {
    if(clickMap&board[0])
      return 0
    return 1
  }
</script>

<div class="flex flex-col items-center justify-center h-lvh">
  <canvas bind:this={canvas} class="main-canvas" on:mousemove={handleMouseMove} on:click={handleClick} on:mousedown={dragStart} on:mouseup={dragEnd} on:mouseleave={directions}></canvas>
  <canvas bind:this={dragCanvas} class="drag-canvas"></canvas>
</div>
<div>{assigner}</div>
<div>{hideNow}</div>
<img src={blackPawnSrc} alt="Pawn"/>

<style>
  .main-canvas {
    border: 10px solid black;
    z-index: 1;
    position: absolute;
  }
  .drag-canvas {
    z-index: 2;
    position: absolute;
    pointer-events: none;
  }
</style>
