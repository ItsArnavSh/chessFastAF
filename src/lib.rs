mod moves;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_moves(packed: Vec<u32>) -> Vec<u32> {
    let board = breakToBoard(packed.clone());
    let click:u64 = ((packed[30] as u64)<<32)+(packed[31] as u64);
    return u64tobigintarray(moves::generate_moves(board,click))
}
fn u64tobigintarray(num: u64) -> Vec<u32> {
    // Get the higher 32 bits
    let high_bits = (num >> 32) as u32;
    // Get the lower 32 bits by applying a mask
    let low_bits = (num & 0xFFFFFFFF) as u32;
    let arr: Vec<u32> = vec![high_bits, low_bits];
    return arr;
}
fn breakToBoard(oldBoard:Vec<u32>)->Vec<u64>
{
    let mut board:Vec<u64> = vec![];
    for i in 0..15
    {
        board.push(((oldBoard[2*i] as u64)<<32)+(oldBoard[2*i+1] as u64))
    }
    return board;
}