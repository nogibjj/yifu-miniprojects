use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut board = vec![' '; 9];
    let mut player = 'X';

    loop {
        print_board(&board);
        let index = if player == 'X' {
            get_input(&player)
        } else {
            get_computer_move(&board)
        };
        board[index] = player;

        if check_win(&board, player) {
            println!("{} wins!", player);
            break;
        }

        if board.iter().all(|&x| x != ' ') {
            println!("Tie game!");
            break;
        }

        player = if player == 'X' { 'O' } else { 'X' };
    }
}

fn print_board(board: &[char]) {
    println!(" {} | {} | {}", board[0], board[1], board[2]);
    println!("---+---+---");
    println!(" {} | {} | {}", board[3], board[4], board[5]);
    println!("---+---+---");
    println!(" {} | {} | {}", board[6], board[7], board[8]);
}

fn get_input(player: &char) -> usize {
    loop {
        print!("{}'s turn: ", player);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<usize>() {
            Ok(x) if x < 9 => return x,
            _ => println!("Invalid input!"),
        }
    }
}

fn get_computer_move(board: &[char]) -> usize {
    let mut rng = rand::thread_rng();
    loop {
        let index = rng.gen_range(0..9);
        if board[index] == ' ' {
            return index;
        }
    }
}

fn check_win(board: &[char], player: char) -> bool {
    let win_patterns = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8], // Rows
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8], // Columns
        [0, 4, 8],
        [2, 4, 6], // Diagonals
    ];

    win_patterns
        .iter()
        .any(|&[a, b, c]| board[a] == player && board[b] == player && board[c] == player)
}
