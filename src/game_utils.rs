pub fn check_winner(board: Vec<Vec<char>>) -> char {
    return if is_win(board.clone(), 'X') {
        'X'
    } else if is_win(board.clone(), 'O') {
        'O'
    } else if is_draw(board.clone()) {
        'D'
    } else {
        ' '
    }
}

fn is_win(board: Vec<Vec<char>>, player_char: char) -> bool {
    let x_length = board.len();
    for i in 0..x_length {
        // check rows
        if board[i][0] == player_char && board[i][1] == player_char && board[i][2] == player_char {
            return true;
        }
        // check columns
        if board[0][i] == player_char && board[1][i] == player_char && board[2][i] == player_char {
            return true;
        }
    }
    // check diagonals
    if board[0][0] == player_char && board[1][1] == player_char && board[2][2] == player_char {
        return true;
    }
    if board[0][2] == player_char && board[1][1] == player_char && board[2][0] == player_char {
        return true;
    }
    // no win condition found
    return false;
}

fn is_draw(board: Vec<Vec<char>>) -> bool {
    let mut filled_count = 0;
    let x_length = board.len();
    let y_length = board[0].len();
    for i in 0..x_length {
        for j in 0..y_length {
            if board[i][j] != ' ' {
                filled_count += 1;
            }
        }
    }
    if filled_count == x_length * y_length {
        return true;
    }
    return false
}