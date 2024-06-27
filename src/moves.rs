//This rust module is the first one
//This will return the list of moves for any given piece


pub fn generate_moves(board:Vec<u64>,click:u64)->u64
{
    //First we see what kind of piece it is, and we work accordingly
    let mut typ:usize=0;
    for i in 2..14
    {
        if (board[i]&click)!=0
        {
            typ = i;
            break;
        }
    }
    let friend = if typ<8 {board[0]} else {board[1]};
    let enemy = if typ<8 {board[1]} else {board[0]};
    match typ
    {
        2|8=>{
            let bitmap = king(click,friend);
            return bitmap
        }
        3|9=>{
            let bitmap = queen(click,friend,enemy);
            return bitmap
        }
        4|10=>{
            let bitmap = rook(click,friend,enemy);
            return bitmap
        }
        5|11=>{
            let bitmap = knight(click,friend);
            return bitmap
        }
        6|12=>{
            let bitmap = bishop(click,friend,enemy);
            return bitmap
        }
        7|13=>{
            let is_white = 
            if board[14]&0b1!=0
             {true}
             else
             {false};
            let bitmap = pawn(click,friend,enemy,is_white);
            return bitmap;
        }
        _=>{return typ as u64}
    }
}
fn map_to_num(bitmap:u64)->u8
{
    let mut bitmap = bitmap;
    for i in 1..65
    {
        if (bitmap&1)==1
                {return 64-i;}
            bitmap=bitmap>>1;
    }
    0
}

fn pawn(click: u64, friend: u64, enemy: u64, is_white: bool) -> u64 {
    let mut moves: u64 = 0;

    // Shifts for one and two steps forward
    let one_step = if is_white { click << 8 } else { click >> 8 };
    let two_steps = if is_white { click << 16 } else { click >> 16 };

    // Start ranks for white and black pawns
    let start_rank = if is_white { 0x000000000000FF00 } else { 0x00FF000000000000 };

    // Single move forward (one step)
    if (one_step & (friend | enemy)) == 0 {
        moves |= one_step;

        // Double move forward (two steps) from starting rank
        if (click & start_rank) != 0 && (two_steps & (friend | enemy)) == 0 {
            moves |= two_steps;
        }
    }

    // Capture moves
    let left_capture = if is_white {
        (click << 9) & 0xFEFEFEFEFEFEFEFE // Shift left-up for white
    } else {
        (click >> 7) & 0x7F7F7F7F7F7F7F7F // Shift right-down for black
    };

    let right_capture = if is_white {
        (click << 7) & 0x7F7F7F7F7F7F7F7F // Shift right-up for white
    } else {
        (click >> 9) & 0xFEFEFEFEFEFEFEFE // Shift left-down for black
    };

    // Apply captures only if enemy pieces are present
    moves |= left_capture & enemy;
    moves |= right_capture & enemy;

    // Ensure moves do not include friend's pieces
    moves &= !friend;

    return moves;
}

fn king(click:u64, friend:u64)-> u64
{
    //Watch this
    /*
    11100000
    10100000
    11100000
    */
    //This is called a cool algorithm
    //We find the distance between this bitboard shown above and our piece
    //Then drag it there, like a lookup blueprint
    let mut king_moves:u128 = 0b1110000010100000111000000000000000000000000000000000000000000000<<64;//the same pattern
    //We need to shift the king to the bitboard
    king_moves=king_moves>>(map_to_num(click)+55u8);
    let mut king_moves:u64 = assist::board_bound_check(king_moves) as u64;
    king_moves&=!friend;
    king_moves = assist::mask(king_moves,click);
    return king_moves;
}

fn knight(click:u64, friend:u64)-> u64
{
    //Watch this
    /*
    01010000
    10001000
    00o00000
    10001000
    01010000
    */
    let mut knight_moves:u128 = 0b0101000010001000000000001000100001010000000000000000000000000000<<64;
    //It is u128 to avoid overflow
    //We will shift the knight coord
    knight_moves=knight_moves>>(map_to_num(click)+46u8);
    let mut knight_moves = assist::board_bound_check(knight_moves);
    knight_moves&=!friend;//Removing friends under attack

    knight_moves = assist::mask(knight_moves,click);
   return knight_moves;
}

fn rook(click:u64, friend:u64, enemy:u64)-> u64
{
    let mut moves = assist::find_moves(click,1,0,enemy,friend);//Left
    moves |= assist::find_moves(click,0,1,enemy,friend);//Up
    moves |= assist::find_moves(click,-1,0,enemy,friend);//Right
    moves |= assist::find_moves(click,0,-1,enemy,friend);//Down
    moves
}

fn bishop(click:u64, friend:u64, enemy:u64)-> u64
{
    let mut moves = assist::find_moves(click,1,1,enemy,friend);
    moves |= assist::find_moves(click,-1,-1,enemy,friend);
    moves |= assist::find_moves(click,-1,1,enemy,friend);
    moves |= assist::find_moves(click,1,-1,enemy,friend);
    moves
}
#[warn(dead_code)]
fn queen(click:u64, friend:u64, enemy:u64)-> u64
{
    let moves:u64 = rook(click,friend,enemy)|bishop(click,friend,enemy);
    moves
}

pub fn check(location:u64,rook_piece:u64,queen_piece:u64,bishop_piece:u64,pawn_piece:u64,king_piece:u64,knight_piece:u64,enemy:u64,friend:u64)->bool
{
    let perpendicular:u64 = rook(location,friend,enemy);
    if perpendicular & (rook_piece | queen_piece) !=0
    {
        return true;
    }
    let parallel:u64 = bishop(location,friend,enemy);
    if parallel & (bishop_piece|queen_piece) != 0
    {
        return true;
    }
    if king(location,friend) & king_piece != 0
    {
        return true;
    }
    if knight(location,friend) & knight_piece != 0
    {
        return true;
    }
    return false;
}

mod assist
{
    pub fn mask(moves:u64,click:u64)->u64
    {
        let mut moves = moves;
        //Now we will perform an antiLeak
        //Now there is an issue
        //The piece when at corner would project its pieces and they got leaked to the other end
        //As the bitboard is just one huge single dimensional array
        //To prevent this, we see which side the piece is on and clear off the other side
        //This bit notation below refers to the second half of the board
        //To see if the piece is located in the second half
        let left_leak:u64 =  0b0011111100111111001111110011111100111111001111110011111100111111;
        let right_leak:u64 = 0b1111110011111100111111001111110011111100111111001111110011111100;
        if click & 0b1111000011110000111100001111000011110000111100001111000011110000 != 0
            {moves&=right_leak;}
        else
            {moves&=left_leak;}
        moves
    }
    pub fn board_bound_check(moves:u128)->u64
    {
        let board_bounds:u128= 0b1111111111111111111111111111111111111111111111111111111111111111;
        return (moves&board_bounds) as u64;
    }
    pub fn find_moves(moves: u64, xdirection: i8, ydirection: i8, enemy: u64, friend: u64) -> u64 {
        let mut result: u64 = 0;
        let mut moves = moves;
    
        // Define boundaries for left and right edges
        let xboundary_left: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;
        let xboundary_right: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
    
        // Define boundaries for top and bottom edges
        let yboundary_top: u64 = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        let yboundary_bottom: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
    
        loop {
            // Horizontal movement
            if xdirection > 0 {
                if moves & xboundary_right != 0 { // Check if moving right would go out of bounds
                    break;
                }
                moves >>= xdirection as u8;
            } else if xdirection < 0 {
                if moves & xboundary_left != 0 { // Check if moving left would go out of bounds
                    break;
                }
                moves <<= (-xdirection) as u8;
            }
    
            // Vertical movement
            if ydirection > 0 {
                if moves & yboundary_bottom != 0 { // Check if moving down would go out of bounds
                    break;
                }
                moves >>= 8 * ydirection as u8;
            } else if ydirection < 0 {
                if moves & yboundary_top != 0 { // Check if moving up would go out of bounds
                    break;
                }
                moves <<= 8 * (-ydirection) as u8;
            }
    
            // Check if move lands on a friendly piece
            if moves & friend != 0 {
                break;
            } else if moves & enemy != 0 {
                result |= moves;
                break;
            }
    
            result |= moves;
        }
    
        result
    }

    fn map_to_maps(moves: u64) -> Vec<u64> {
        let mut results = Vec::new();
        let mut bit_pos = 0;
        
        while moves >> bit_pos != 0 {
            if (moves & (1 << bit_pos)) != 0 {
                results.push(1 << bit_pos);
            }
            bit_pos += 1;
        }
        
        results
    }
    
}

