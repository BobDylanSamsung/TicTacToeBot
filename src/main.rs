mod board_utils;
mod game_utils;
mod user_utils;

fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLS: usize = 3;
    board_utils::clear_screen();
    let mut board = board_utils::create_board(TOTAL_ROWS, TOTAL_COLS);
    let human_char = user_utils::ask_player_char();

    board_utils::print_board(board.clone());
    loop {
        let human_move = user_utils::ask_player_move(board.clone(), human_char);
        board_utils::fill_box(&mut board, human_move[0], human_move[1], human_char);
        board_utils::print_board(board.clone());

        if game_utils::check_winner(board.clone()) == human_char {
            return;
        }
    }
}
