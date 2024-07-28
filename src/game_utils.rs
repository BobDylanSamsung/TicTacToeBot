pub fn check_winner(board: Vec<Vec<char>>, active_char: char) -> char {
    return if is_win(board.clone(), active_char) {
        active_char
    } else if board.iter().all(|v| v.iter().all(|&x| x != ' ')) {
        'D'
    } else {
        ' '
    };
}

pub fn is_win(board: Vec<Vec<char>>, active_char: char) -> bool {
    let size = board.len();

    // check rows and cols
    for i in 0..board.len() {
        if board[i].iter().all(|&x| x == active_char) {
            return true;
        }
        if board.iter().all(|v| v[i] == active_char) {
            return true;
        }
    }
    // Check diagonals
    if (0..size).all(|i| board[i][i] == active_char) {
        return true;
    }
    if (0..size).all(|i| board[i][size - 1 - i] == active_char) {
        return true;
    }

    return false;
}
