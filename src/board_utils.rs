use tabled::builder::Builder;
use tabled::settings::Style;

/// ANSI escape codes for clearing screen
/// \x1B:  introduce an ANSI escape sequence
/// [2J:   escape sequence to clear the entire screen
/// [1;1H: escape sequence to move the cursor to the home position
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn fill_box(board: &mut Vec<Vec<char>>, x: usize, y: usize, player_char: char) {
    if let Some(row) = board.get_mut(x) {
        if let Some(element) = row.get_mut(y) {
            *element = player_char;
        }
    }
}

pub fn print_board(board: Vec<Vec<char>>) {
    let x_length = board.len();
    let y_length = board[0].len();

    let mut builder = Builder::default();
    for i in 0..x_length {
        let mut row: Vec<char> = Vec::new();
        for j in 0..y_length {
            if board[i][j] == ' ' {
                let box_num = i * x_length + j + 1;
                let box_num_char = (b'0' + box_num as u8) as char;
                row.push(box_num_char);
            } else {
                row.push(board[i][j]);
            }
        }
        builder.push_record(row);
    }
    let table = builder.build().with(Style::modern()).to_string();
    println!("{}", table);
}

pub fn create_board(total_rows: usize, total_cols: usize) -> Vec<Vec<char>> {
    let mut array: Vec<Vec<char>> = Vec::new();
    for _ in 0..total_rows {
        let row: Vec<char> = vec![' '; total_cols];
        array.push(row);
    }
    return array;
}
