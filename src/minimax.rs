use crate::game_utils;

fn minimax(
    board: &mut Vec<Vec<char>>,
    is_maximizing: bool,
    depth: isize,
    ai_char: char,
    human_char: char,
) -> isize {
    let result = game_utils::check_winner(board.clone());
    if result != ' ' {
        return get_score_from_result(result, human_char);
    }

    let x_length = board.len();
    let y_length = board[0].len();

    if is_maximizing {
        let mut best_score = -100;
        for i in 0..x_length {
            for j in 0..y_length {
                if board[i][j] == ' ' {
                    board[i][j] = ai_char;
                    let score = minimax(board, false, depth + 1, ai_char, human_char);
                    board[i][j] = ' ';
                    if score > best_score {
                        best_score = score;
                    }
                }
            }
        }
        return best_score - depth;
    } else {
        let mut best_score = 100;
        for i in 0..x_length {
            for j in 0..y_length {
                if board[i][j] == ' ' {
                    board[i][j] = human_char;
                    let score = minimax(board, true, depth + 1, ai_char, human_char);
                    board[i][j] = ' ';
                    if score < best_score {
                        best_score = score;
                    }
                }
            }
        }
        return best_score - depth;
    }
}

fn get_score_from_result(res: char, human_char: char) -> isize {
    if res == 'D' {
        return 0;
    } else if res == human_char {
        return -100;
    } else {
        return 100;
    }
}

pub fn ai_best_move(board: &mut Vec<Vec<char>>, ai_char: char, human_char: char) -> [usize; 2] {
    let x_length = board.len();
    let y_length = board[0].len();
    let mut best_score = -100;
    let mut best_move: [usize; 2] = Default::default();
    for i in 0..x_length {
        for j in 0..y_length {
            if board[i][j] == ' ' {
                board[i][j] = ai_char;
                let score = minimax(board, false, 1, ai_char, human_char);
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
