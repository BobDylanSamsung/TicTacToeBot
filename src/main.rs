mod board_utils;
mod game_utils;

fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLS: usize = 3;

    board_utils::clear_screen();
    let mut board = board_utils::create_board(TOTAL_ROWS, TOTAL_COLS);
    board_utils::print_board(board.clone());
    board_utils::fill_box(&mut board, 0, 0, 'X');
    board_utils::fill_box(&mut board, 1, 1, 'X');
    board_utils::fill_box(&mut board, 2, 2, 'X');
    board_utils::print_board(board.clone());
    let winner = game_utils::check_winner(board);
    println!("The winner is {}", winner);
}

