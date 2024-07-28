pub fn check_winner(board: Vec<Vec<char>>, active_char: char, turn_no: usize) -> char {
    return if is_win(board.clone(), active_char) {
        active_char
    } else if turn_no == board.len() * board[0].len() {
        'D'
    } else {
        ' '
    };
}

fn is_win(board: Vec<Vec<char>>, active_char: char) -> bool {
    for i in 0..board.len() {
        if board[i].iter().all(|&x| x == active_char) {
            return true;
        }
        if board.iter().all(|&x| x[i] == active_char) {
            return true;
        }
    }
    if board.iter().enumerate().all(|(i, &x)| x[i][i] == active_char){
        return true;
    }
    if board.iter().enumerate().all(|(i, &x)| x[i][board.len()-i] == active_char){
        return true;
    }
    return false;
}
