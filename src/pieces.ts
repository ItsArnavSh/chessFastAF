import { writable } from "svelte/store";
//Here is how we will store everything in a single array
//0 for all black pieces
//1 for all white pieces
//2,3,4,5,6,7 for Black King Queen Rook Knight Bishop Pawn
//8,9,10,11,12,13 for White King Queen Rook Knight Bishop Pawn
//14 is the special one, here is all it can do its bits are in the order: [wwwwwwwwooooooooabcdt]
// - t is the turn, 1 means white and 0 means black
//abcd refer to castling avaibility, all are 1 by default
//all ws and bs represent en passant squears, as there can be 8 such ones, they will be mapped. All are 0 by default
export let bitBoard = writable(
    [
        0b1111111111111111000000000000000000000000000000000000000000000000n,//All Black Pieces
        0b0000000000000000000000000000000000000000000000001111111111111111n,//All White Pieces
        0b0000100000000000000000000000000000000000000000000000000000000000n,//Black King
        0b0001000000000000000000000000000000000000000000000000000000000000n,//Black Queen
        0b1000000100000000000000000000000000000000000000000000000000000000n,//Black Rook
        0b0100001000000000000000000000000000000000000000000000000000000000n,//Black Knight
        0b0010010000000000000000000000000000000000000000000000000000000000n,//Black Bishop
        0b0000000011111111000000000000000000000000000000000000000000000000n,//Black Pawn
        0b0000000000000000000000000000000000000000000000000000000000001000n,//White King
        0b0000000000000000000000000000000000000000000000000000000000010000n,//White Queen
        0b0000000000000000000000000000000000000000000000000000000010000001n,//White Rook
        0b0000000000000000000000000000000000000000000000000000000001000010n,//White Knight
        0b0000000000000000000000000000000000000000000000000000000000100100n,//White Bishop
        0b0000000000000000000000000000000000000000000000001111111100000000n,//White Pawn
        0b000000000000000011110//The Special One 
    ]
)