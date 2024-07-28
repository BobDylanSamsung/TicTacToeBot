use crate::board_utils::fill_box;
use crate::game_utils::check_winner;

mod board_utils;
mod game_utils;
mod user_utils;
mod minimax;

fn main() {
    const TOTAL_ROWS: usize = 3;
    const TOTAL_COLS: usize = 3;
    const MAX_TURNS: usize = TOTAL_ROWS * TOTAL_COLS;

    let mut board: Vec<Vec<char>> = board_utils::create_board(TOTAL_ROWS, TOTAL_COLS);
    let mut winner: char = ' ';
    let mut turn_no: usize = 0;

    let human_char = user_utils::ask_player_char();

    while turn_no < MAX_TURNS && winner == ' ' {
        board_utils::clear_screen();
        board_utils::print_board(board.clone());
        let active_char: char = if turn_no % 2 == 0 { 'X' } else { 'O' };

        let current_move: [usize; 2] = if active_char == human_char {
            user_utils::ask_player_move(board.clone(), human_char)
        } else {
            minimax::ai_best_move(&mut board, active_char, human_char)
        };

        fill_box(&mut board, current_move[0], current_move[1], active_char);
        turn_no += 1;
        winner = check_winner(board.clone());

    }

    board_utils::clear_screen();
    board_utils::print_board(board.clone());
    println!("Winner is: {}!", winner);
}