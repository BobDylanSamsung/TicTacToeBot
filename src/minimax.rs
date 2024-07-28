use crate::game_utils;

fn minimax(
    board: &mut Vec<Vec<char>>,
    depth: usize,
    is_maxing: bool,
    active_char: char,
    human_char: char,
) -> f32 {
    let other_char: char = if active_char == 'X' { 'O' } else { 'X' };

    // check terminating state
    let result = game_utils::check_winner(board.clone(), other_char);
    if result != ' ' {
        return get_score_from_result(result, human_char);
    }

    let mut best_score: f32 = if is_maxing {
        f32::NEG_INFINITY
    } else {
        f32::INFINITY
    };

    // recurse for each available move
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == ' ' {
                board[i][j] = active_char;
                let score = minimax(board, depth + 1, !is_maxing, other_char, human_char);
                board[i][j] = ' ';
                if (score > best_score && is_maxing) || (score < best_score && !is_maxing) {
                    best_score = score;
                }
            }
        }
    }

    // prefer moves that win quicker
    return best_score - (depth as f32);
}

fn get_score_from_result(res: char, human_char: char) -> f32 {
    if res == 'D' {
        return 0.0;
    } else if res == human_char {
        return -100.0;
    } else {
        return 100.0;
    }
}

pub fn ai_best_move(
    board: &mut Vec<Vec<char>>,
    ai_char: char,
    human_char: char,
    turn_no: usize,
) -> [usize; 2] {
    // if it is the first turn hard code best move to avoid expensive computation
    if turn_no == 0 {
        return [0, 0];
    }
    let x_length = board.len();
    let y_length = board[0].len();

    let mut best_score = f32::NEG_INFINITY;
    let mut best_move: [usize; 2] = Default::default();

    // run minimax for each possible move for computer
    for i in 0..x_length {
        for j in 0..y_length {
            if board[i][j] == ' ' {
                board[i][j] = ai_char;
                let score = minimax(board, 0, false, human_char, human_char);
                board[i][j] = ' ';
                if score > best_score {
                    best_score = score;
                    best_move = [i, j];
                }
            }
        }
    }

    return best_move;
}
