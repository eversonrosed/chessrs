use chessrs::boards;
use chessrs::boards::Board;
use chessrs::mapping::Square;
use chessrs::pieces::{move_occlusion, rook_moves};

fn main() {
    let board = Board::init();
    let rma = &boards::ROOK_PREMOVE_TBL;
    println!("{}", rma[0]);
    println!("{}", move_occlusion(Square::A1, rma[0], board.occupancy()));
}
